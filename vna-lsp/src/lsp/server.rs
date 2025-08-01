use crate::lsp::diagnostics::DiagnosticsProvider;
use crate::types::VnaDocument;
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub struct VnaLanguageServer {
    client: Client,
    documents: RwLock<HashMap<Url, VnaDocument>>,
    diagnostics_provider: DiagnosticsProvider,
}

impl VnaLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: RwLock::new(HashMap::new()),
            diagnostics_provider: DiagnosticsProvider::new(),
        }
    }

    pub async fn run() -> Result<()> {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let (service, socket) = LspService::new(|client| VnaLanguageServer::new(client));
        Server::new(stdin, stdout, socket).serve(service).await;

        Ok(())
    }

    async fn update_diagnostics(&self, uri: &Url, document: &VnaDocument) {
        let diagnostics = self.diagnostics_provider.provide_diagnostics(document);
        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for VnaLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["[".to_string(), "|".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "VNA Language Server initialized")
            .await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;

        match crate::parser::parse(&content) {
            Ok(document) => {
                self.documents.write().await.insert(uri.clone(), document.clone());
                self.update_diagnostics(&uri, &document).await;
            }
            Err(err) => {
                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 1 },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    source: Some("vna".to_string()),
                    message: format!("Parse error: {}", err),
                    related_information: None,
                    tags: None,
                    code_description: None,
                    data: None,
                };
                self.client
                    .publish_diagnostics(uri, vec![diagnostic], None)
                    .await;
            }
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next() {
            match crate::parser::parse(&change.text) {
                Ok(document) => {
                    self.documents.write().await.insert(uri.clone(), document.clone());
                    self.update_diagnostics(&uri, &document).await;
                }
                Err(err) => {
                    let diagnostic = Diagnostic {
                        range: Range {
                            start: Position { line: 0, character: 0 },
                            end: Position { line: 0, character: 1 },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        source: Some("vna".to_string()),
                        message: format!("Parse error: {}", err),
                        related_information: None,
                        tags: None,
                        code_description: None,
                        data: None,
                    };
                    self.client
                        .publish_diagnostics(uri, vec![diagnostic], None)
                        .await;
                }
            }
        }
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {}

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.write().await.remove(&params.text_document.uri);
    }

    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let documents = self.documents.read().await;
        if let Some(document) = documents.get(&uri) {
            return Ok(crate::lsp::hover::provide_hover(document, position));
        }

        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let documents = self.documents.read().await;
        if let Some(document) = documents.get(&uri) {
            let completions = crate::lsp::completion::provide_completions(document, position);
            return Ok(Some(CompletionResponse::Array(completions)));
        }

        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> LspResult<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;

        let documents = self.documents.read().await;
        if let Some(document) = documents.get(&uri) {
            match crate::formatter::format(document) {
                Ok(formatted_text) => {
                    let edit = TextEdit {
                        range: Range {
                            start: Position { line: 0, character: 0 },
                            end: Position { line: u32::MAX, character: 0 },
                        },
                        new_text: formatted_text,
                    };
                    return Ok(Some(vec![edit]));
                }
                Err(_) => return Ok(None),
            }
        }

        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> LspResult<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        let documents = self.documents.read().await;
        if let Some(document) = documents.get(&uri) {
            let symbols = crate::lsp::handlers::create_document_symbols(document);
            return Ok(Some(DocumentSymbolResponse::Nested(symbols)));
        }

        Ok(None)
    }

    async fn code_action(&self, params: CodeActionParams) -> LspResult<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;

        let documents = self.documents.read().await;
        if let Some(document) = documents.get(&uri) {
            let actions = crate::lsp::handlers::create_code_actions(document, &params.range);
            return Ok(Some(actions));
        }

        Ok(None)
    }
}