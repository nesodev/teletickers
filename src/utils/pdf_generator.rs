use crate::domain::Comprobante;
use crate::error::{AppError, AppResult};
use printpdf::*;

pub struct ComprobanteData {
    pub empresa_ruc: String,
    pub empresa_razon_social: String,
    pub empresa_direccion: String,
}

pub fn generate_comprobante_pdf(comprobante: &Comprobante, empresa_data: &ComprobanteData) -> AppResult<Vec<u8>> {
    let mut doc = PdfDocument::new("Comprobante");

    let font_bytes: &[u8] = include_bytes!("../../assets/Roboto-Italic-VariableFont_wdth,wght.ttf");
    let mut warnings = Vec::new();
    let font = ParsedFont::from_bytes(font_bytes, 0, &mut warnings).ok_or(AppError::InternalServerError("Error cargando fuente".to_string()))?;
    let font_id = doc.add_font(&font);

    let page_ops = vec![
        Op::SetTextCursor {
            pos: Point::new(Mm(20.0), Mm(270.0)).into(),
        },
        Op::WriteText {
            items: vec![TextItem::Text(format!("{} ELECTRÓNICA", comprobante.tipo.as_str().to_uppercase()))],
            font: font_id.clone(),
        },
        Op::AddLineBreak,
        Op::WriteText {
            items: vec![TextItem::Text(format!("N° {}", comprobante.numero_completo()))],
            font: font_id.clone(),
        },
        Op::AddLineBreak,
        Op::WriteText {
            items: vec![TextItem::Text(empresa_data.empresa_razon_social.clone())],
            font: font_id.clone(),
        },
    ];

    let page = PdfPage::new(Mm(210.0), Mm(297.0), page_ops);

    let mut warnings = Vec::new();
    let pdf_bytes: Vec<u8> = doc.with_pages(vec![page]).save(&PdfSaveOptions::default(), &mut warnings);
    Ok(pdf_bytes)
}
