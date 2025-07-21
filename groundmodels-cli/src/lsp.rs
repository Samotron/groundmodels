use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use serde_json;
use std::collections::HashMap;

pub struct Backend {
    client: Client,
    documents: tokio::sync::RwLock<HashMap<Url, String>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    async fn validate_agsi_document(&self, uri: &Url, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Try to parse as JSON
        match serde_json::from_str::<serde_json::Value>(text) {
            Ok(json_value) => {
                // Check for required AGSi fields
                if !json_value.get("agsFile").is_some() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: 0, character: 0 },
                            end: Position { line: 0, character: 1 },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("agsi-lsp".to_string()),
                        message: "Missing required 'agsFile' field".to_string(),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }

                if !json_value.get("agsiModel").is_some() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: 0, character: 0 },
                            end: Position { line: 0, character: 1 },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("agsi-lsp".to_string()),
                        message: "Missing required 'agsiModel' field".to_string(),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }

                // Validate parameter value structure
                if let Some(agsi_model) = json_value.get("agsiModel").and_then(|v| v.as_array()) {
                    for (model_idx, model) in agsi_model.iter().enumerate() {
                        if let Some(elements) = model.get("agsiModelElement").and_then(|v| v.as_array()) {
                            for (elem_idx, element) in elements.iter().enumerate() {
                                if let Some(params) = element.get("agsiDataParameterValue").and_then(|v| v.as_array()) {
                                    for (param_idx, param) in params.iter().enumerate() {
                                        if !param.get("codeID").is_some() {
                                            diagnostics.push(Diagnostic {
                                                range: Range {
                                                    start: Position { line: (model_idx + elem_idx + param_idx) as u32, character: 0 },
                                                    end: Position { line: (model_idx + elem_idx + param_idx) as u32, character: 50 },
                                                },
                                                severity: Some(DiagnosticSeverity::WARNING),
                                                code: None,
                                                code_description: None,
                                                source: Some("agsi-lsp".to_string()),
                                                message: "Parameter missing 'codeID' field".to_string(),
                                                related_information: None,
                                                tags: None,
                                                data: None,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 1 },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("agsi-lsp".to_string()),
                    message: format!("Invalid JSON: {}", e),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }

        diagnostics
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "AGSi Language Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), "\"".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "AGSi Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        
        self.documents.write().await.insert(uri.clone(), text.clone());
        
        let diagnostics = self.validate_agsi_document(&uri, &text).await;
        
        self.client
            .publish_diagnostics(uri, diagnostics, Some(params.text_document.version))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.content_changes[0].text.clone();
        
        self.documents.write().await.insert(uri.clone(), text.clone());
        
        let diagnostics = self.validate_agsi_document(&uri, &text).await;
        
        self.client
            .publish_diagnostics(uri, diagnostics, Some(params.text_document.version))
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let agsi_completions = vec![
            CompletionItem {
                label: "UnitWeight".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Unit weight parameter (kN/m³)".to_string()),
                insert_text: Some("\"UnitWeight\"".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "EffectiveFrictionAngle".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Effective friction angle (degrees)".to_string()),
                insert_text: Some("\"EffectiveFrictionAngle\"".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "EffectiveCohesion".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Effective cohesion (kPa)".to_string()),
                insert_text: Some("\"EffectiveCohesion\"".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "UndrainedShearStrength".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Undrained shear strength (kPa)".to_string()),
                insert_text: Some("\"UndrainedShearStrength\"".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "YoungsModulus".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Young's modulus (kPa)".to_string()),
                insert_text: Some("\"YoungsModulus\"".to_string()),
                ..Default::default()
            },
        ];

        Ok(Some(CompletionResponse::Array(agsi_completions)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        
        if let Some(text) = self.documents.read().await.get(uri) {
            // Simple hover support - could be enhanced with more sophisticated parsing
            let hover_text = if text.contains("UnitWeight") {
                "Unit weight of soil material in kN/m³"
            } else if text.contains("EffectiveFrictionAngle") {
                "Effective friction angle in degrees"
            } else if text.contains("EffectiveCohesion") {
                "Effective cohesion in kPa"
            } else if text.contains("UndrainedShearStrength") {
                "Undrained shear strength in kPa"
            } else if text.contains("YoungsModulus") {
                "Young's modulus in kPa"
            } else {
                return Ok(None);
            };

            return Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(hover_text.to_string())),
                range: None,
            }));
        }

        Ok(None)
    }
}