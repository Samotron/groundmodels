use crate::{GroundModel, SoilLayer, SoilType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripLogEntry {
    pub top_level: f64,
    pub bottom_level: f64,
    pub thickness: f64,
    pub reference: String,
    pub typical_description: Option<String>,
    pub behavior: SoilType,
    pub unit_weight: Option<f64>,
    pub phi_prime_deg: Option<f64>,
    pub c_prime: Option<f64>,
    pub cu: Option<f64>,
    pub gw_within_layer: bool,
    pub gw_above_top: bool,
    pub sigma_v_total_mid: Option<f64>,
    pub u_mid: Option<f64>,
    pub sigma_v_prime_mid: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BuildStripLogOptions {
    pub include_stresses: bool,
    pub stress_dz: f64,
}

impl Default for BuildStripLogOptions {
    fn default() -> Self {
        BuildStripLogOptions {
            include_stresses: false,
            stress_dz: 0.05,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripLogRenderOptions {
    pub column_width_px: u32,
    pub px_per_meter: Option<f64>,
    pub left_margin_px: u32,
    pub right_margin_px: u32,
    pub top_margin_px: u32,
    pub bottom_margin_px: u32,
    pub tick_every_meters: f64,
    pub show_grid: bool,
    pub show_labels: bool,
    pub axis_unit_label: String,
    pub colors: Option<StripLogColors>,
    pub font_family: String,
    pub title: Option<String>,
}

impl Default for StripLogRenderOptions {
    fn default() -> Self {
        StripLogRenderOptions {
            column_width_px: 220,
            px_per_meter: None,
            left_margin_px: 64,
            right_margin_px: 64,
            top_margin_px: 28,
            bottom_margin_px: 28,
            tick_every_meters: 1.0,
            show_grid: true,
            show_labels: true,
            axis_unit_label: "m".to_string(),
            colors: None,
            font_family: "Segoe UI, Arial, sans-serif".to_string(),
            title: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripLogColors {
    pub cohesive: Option<String>,
    pub granular: Option<String>,
    pub rock: Option<String>,
}

impl Default for StripLogColors {
    fn default() -> Self {
        StripLogColors {
            cohesive: Some("#B8906B".to_string()),
            granular: Some("#F6D04D".to_string()),
            rock: Some("#9BA3AD".to_string()),
        }
    }
}

impl GroundModel {
    pub fn to_strip_log(&self, opts: BuildStripLogOptions) -> Vec<StripLogEntry> {
        let mut layers = self.soil_layers.clone();
        layers.sort_by(|a, b| b.top_level.partial_cmp(&a.top_level).unwrap());

        let mut entries = Vec::new();
        for layer in layers {
            let reference = layer_reference(&layer);
            let params = self.get_soil_params(&reference).cloned();
            let mid = match layer.base_level {
                Some(base) => (layer.top_level + base) / 2.0,
                None => layer.top_level,
            };
            let gw = self.groundwater;
            let base = layer.base_level.unwrap_or(layer.top_level);
            let gw_within =
                (gw >= base && gw <= layer.top_level) || (gw <= layer.top_level && gw >= base);
            let gw_above_top = gw > layer.top_level;

            let mut entry = StripLogEntry {
                top_level: layer.top_level,
                bottom_level: base,
                thickness: (layer.top_level - base).abs(),
                reference: reference.clone(),
                typical_description: if layer.typical_description.is_empty() {
                    None
                } else {
                    Some(layer.typical_description.clone())
                },
                behavior: params
                    .as_ref()
                    .map(|p| p.behaviour)
                    .unwrap_or(SoilType::Granular),
                unit_weight: params.as_ref().map(|p| p.unit_weight),
                phi_prime_deg: params
                    .as_ref()
                    .and_then(|p| p.phi_prime)
                    .map(|phi| phi.to_degrees()),
                c_prime: params.as_ref().and_then(|p| p.c_prime),
                cu: params.as_ref().and_then(|p| p.cu),
                gw_within_layer: gw_within,
                gw_above_top,
                sigma_v_total_mid: None,
                u_mid: None,
                sigma_v_prime_mid: None,
            };

            if opts.include_stresses {
                let sigma_v = integrate_sigma_v_total_at(self, mid, opts.stress_dz);
                let u = self.get_pwp_at_level(mid);
                entry.sigma_v_total_mid = Some(sigma_v);
                entry.u_mid = Some(u);
                entry.sigma_v_prime_mid = Some(sigma_v - u);
            }

            entries.push(entry);
        }

        entries
    }

    pub fn to_strip_log_csv(&self, rows: Option<Vec<StripLogEntry>>) -> String {
        let rows = rows.unwrap_or_else(|| self.to_strip_log(BuildStripLogOptions::default()));
        let headers = [
            "TOP_LEVEL(m)",
            "BOTTOM_LEVEL(m)",
            "THICKNESS(m)",
            "REFERENCE",
            "DESCRIPTION",
            "BEHAVIOR",
            "GAMMA(kN/m3)",
            "PHI_PRIME(deg)",
            "C_PRIME(kPa)",
            "CU(kPa)",
            "GW_WITHIN_LAYER",
            "GW_ABOVE_TOP",
            "SIGMA_V_TOTAL_MID(kPa)",
            "U_MID(kPa)",
            "SIGMA_V_PRIME_MID(kPa)",
        ];

        let mut lines = vec![headers.join(",")];
        for r in rows {
            let desc = r.typical_description.unwrap_or_default();
            lines.push(
                [
                    r.top_level.to_string(),
                    r.bottom_level.to_string(),
                    r.thickness.to_string(),
                    csv_quote(&r.reference),
                    csv_quote(&desc),
                    format!("{:?}", r.behavior),
                    r.unit_weight.map_or("".to_string(), |v| v.to_string()),
                    r.phi_prime_deg.map_or("".to_string(), |v| v.to_string()),
                    r.c_prime.map_or("".to_string(), |v| v.to_string()),
                    r.cu.map_or("".to_string(), |v| v.to_string()),
                    r.gw_within_layer.to_string(),
                    r.gw_above_top.to_string(),
                    r.sigma_v_total_mid
                        .map_or("".to_string(), |v| v.to_string()),
                    r.u_mid.map_or("".to_string(), |v| v.to_string()),
                    r.sigma_v_prime_mid
                        .map_or("".to_string(), |v| v.to_string()),
                ]
                .join(","),
            );
        }

        lines.join("\n")
    }

    pub fn to_ags_geol_csv(&self, hole_id: &str, rows: Option<Vec<StripLogEntry>>) -> String {
        let rows = rows.unwrap_or_else(|| self.to_strip_log(BuildStripLogOptions::default()));
        let headers = ["HOLE_ID", "GEOL_TOP(m)", "GEOL_BASE(m)", "GEOL_DESC"];
        let mut lines = vec![headers.join(",")];
        for r in rows {
            let desc = r
                .typical_description
                .unwrap_or_else(|| format!("{:?}", r.behavior));
            lines.push(
                [
                    csv_quote(hole_id),
                    r.top_level.to_string(),
                    r.bottom_level.to_string(),
                    csv_quote(&desc),
                ]
                .join(","),
            );
        }
        lines.join("\n")
    }

    pub fn render_strip_log_svg(&self, opts: StripLogRenderOptions) -> String {
        let colors = opts.colors.unwrap_or_default();
        let top = self.get_top_level();
        let bottom = self.get_base_level();
        let model_height_m = (top - bottom).abs().max(0.0001);
        let col_height = 600.0;
        let scale = opts.px_per_meter.unwrap_or(col_height / model_height_m);

        let width = opts.left_margin_px + opts.column_width_px + opts.right_margin_px;
        let height = opts.top_margin_px + (model_height_m * scale) as u32 + opts.bottom_margin_px;
        let col_x = opts.left_margin_px as f64;
        let col_y = opts.top_margin_px as f64;
        let col_h = (model_height_m * scale).round();

        let y_for_level = |level: f64| col_y + (top - level) * scale;
        let depth_at = |level: f64| top - level;

        let rows = self.to_strip_log(BuildStripLogOptions::default());

        let mut svg = Vec::new();
        svg.push(format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">",
            width, height, width, height
        ));
        svg.push(format!(
            "<style>.axis{{font:12px {};fill:#333;}}.title{{font:14px {};font-weight:600;fill:#111;}}.tickLabelDepth{{dominant-baseline:middle;text-anchor:end;}}.tickLabelLevel{{dominant-baseline:middle;text-anchor:start;}}.layerDesc{{font:12px {};fill:#222;}}.layerNotes{{font:10px {};fill:#444;}}.legend{{font:12px {};}}</style>",
            opts.font_family, opts.font_family, opts.font_family, opts.font_family, opts.font_family
        ));

        if let Some(title) = &opts.title {
            svg.push(format!(
                "<text class=\"title\" x=\"{}\" y=\"{}\">{}</text>",
                opts.left_margin_px,
                (opts.top_margin_px as f64 * 0.7).max(16.0),
                escape_xml(title)
            ));
        }

        if opts.show_grid {
            let mut m = bottom.ceil();
            while m <= top.floor() {
                let yy = y_for_level(m).round() + 0.5;
                svg.push(format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#eee\" />",
                    col_x,
                    yy,
                    col_x + opts.column_width_px as f64,
                    yy
                ));
                m += opts.tick_every_meters;
            }
        }

        svg.push(format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#fff\" stroke=\"#222\" />",
            col_x, col_y, opts.column_width_px, col_h
        ));

        let mut m = bottom.ceil();
        while m <= top.floor() {
            let yy = y_for_level(m).round() + 0.5;
            let depth = depth_at(m);
            svg.push(format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#222\" />",
                col_x - 6.0,
                yy,
                col_x,
                yy
            ));
            svg.push(format!(
                "<text class=\"axis tickLabelDepth\" x=\"{}\" y=\"{}\">{}</text>",
                col_x - 8.0,
                yy,
                format_number(depth)
            ));
            m += opts.tick_every_meters;
        }
        svg.push(format!(
            "<text class=\"axis\" x=\"{}\" y=\"{}\" text-anchor=\"end\">m bGL</text>",
            col_x - 8.0,
            col_y - 6.0
        ));

        let mut m = bottom.ceil();
        while m <= top.floor() {
            let yy = y_for_level(m).round() + 0.5;
            svg.push(format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#222\" />",
                col_x + opts.column_width_px as f64,
                yy,
                col_x + opts.column_width_px as f64 + 6.0,
                yy
            ));
            svg.push(format!(
                "<text class=\"axis tickLabelLevel\" x=\"{}\" y=\"{}\">{}</text>",
                col_x + opts.column_width_px as f64 + 8.0,
                yy,
                format_number(m)
            ));
            m += opts.tick_every_meters;
        }
        svg.push(format!(
            "<text class=\"axis\" x=\"{}\" y=\"{}\" text-anchor=\"start\">{}</text>",
            col_x + opts.column_width_px as f64 + 8.0,
            col_y - 6.0,
            escape_xml(&opts.axis_unit_label)
        ));

        for row in rows {
            let y_top = y_for_level(row.top_level);
            let y_bot = y_for_level(row.bottom_level);
            let h = (y_bot - y_top).max(0.0);
            let fill = match row.behavior {
                SoilType::Cohesive => colors
                    .cohesive
                    .clone()
                    .unwrap_or_else(|| "#B8906B".to_string()),
                SoilType::Granular => colors
                    .granular
                    .clone()
                    .unwrap_or_else(|| "#F6D04D".to_string()),
                SoilType::Rock => colors.rock.clone().unwrap_or_else(|| "#9BA3AD".to_string()),
            };

            svg.push(format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"#555\" stroke-width=\"0.5\"/>",
                col_x,
                y_top,
                opts.column_width_px,
                h,
                fill
            ));

            if opts.show_labels && h >= 16.0 {
                let inner_x = col_x + 6.0;
                let max_text_width = opts.column_width_px as f64 - 12.0;
                let desc = row
                    .typical_description
                    .clone()
                    .unwrap_or_else(|| format!("{:?}", row.behavior));
                let mut cursor_y = y_top + 6.0 + 12.0;
                for line in wrap_text(&desc, max_text_width, 12.0) {
                    if cursor_y < y_bot - 2.0 {
                        svg.push(format!(
                            "<text class=\"layerDesc\" x=\"{}\" y=\"{}\" dominant-baseline=\"alphabetic\">{}</text>",
                            inner_x,
                            cursor_y,
                            escape_xml(&line)
                        ));
                        cursor_y += 14.0;
                    }
                }
            }
        }

        if self.groundwater <= top && self.groundwater >= bottom {
            let y_gw = y_for_level(self.groundwater).round() + 0.5;
            svg.push(format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#1E90FF\" stroke-width=\"2\" stroke-dasharray=\"6,4\"/>",
                col_x,
                y_gw,
                col_x + opts.column_width_px as f64,
                y_gw
            ));
            svg.push(format!(
                "<text class=\"axis\" x=\"{}\" y=\"{}\" text-anchor=\"end\" fill=\"#1E90FF\">Groundwater</text>",
                col_x - 8.0,
                y_gw - 2.0
            ));
            svg.push(format!(
                "<text class=\"axis\" x=\"{}\" y=\"{}\" text-anchor=\"start\" fill=\"#1E90FF\">{} {}</text>",
                col_x + opts.column_width_px as f64 + 8.0,
                y_gw - 2.0,
                format_number(self.groundwater),
                escape_xml(&opts.axis_unit_label)
            ));
        }

        svg.push("</svg>".to_string());
        svg.join("")
    }
}

fn layer_reference(layer: &SoilLayer) -> String {
    if !layer.unit_reference.is_empty() {
        layer.unit_reference.clone()
    } else {
        layer.reference.clone()
    }
}

fn integrate_sigma_v_total_at(model: &GroundModel, level: f64, dz: f64) -> f64 {
    let top = model.get_top_level();
    if level >= top {
        return 0.0;
    }
    let mut s = 0.0;
    let mut z = top - dz / 2.0;
    while z > level {
        let sp = model.get_params_at_level(z).ok();
        if let Some(params) = sp {
            s += params.unit_weight * dz;
        }
        z -= dz;
    }
    s
}

fn csv_quote(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn wrap_text(text: &str, max_width_px: f64, font_size_px: f64) -> Vec<String> {
    let approx_char_w = 0.6 * font_size_px;
    let max_chars = ((max_width_px / approx_char_w).floor() as usize).max(4);
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut line = String::new();

    for w in words {
        if line.is_empty() {
            line = w.to_string();
        } else if line.len() + 1 + w.len() <= max_chars {
            line.push(' ');
            line.push_str(w);
        } else {
            lines.push(line);
            line = w.to_string();
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn format_number(n: f64) -> String {
    if n.abs() < 1e-6 {
        "0".to_string()
    } else {
        format!("{:.3}", n)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SoilParams;

    fn sample_model() -> GroundModel {
        let mut params1 = SoilParams::default();
        params1.reference = "CL".to_string();
        params1.unit_weight = 19.0;
        params1.behaviour = SoilType::Cohesive;
        params1.phi_prime = Some(22.0_f64.to_radians());

        let mut params2 = SoilParams::default();
        params2.reference = "SA".to_string();
        params2.unit_weight = 18.0;
        params2.behaviour = SoilType::Granular;
        params2.phi_prime = Some(30.0_f64.to_radians());

        let layer1 = SoilLayer::with_all_fields(
            "CL".to_string(),
            1.0,
            Some(-1.0),
            None,
            "Firm clay".to_string(),
            "CL".to_string(),
        );
        let layer2 = SoilLayer::with_all_fields(
            "SA".to_string(),
            -1.0,
            Some(-3.0),
            None,
            "Medium dense sand".to_string(),
            "SA".to_string(),
        );

        let mut model = GroundModel::new(vec![layer1, layer2], vec![params1, params2]);
        model.groundwater = -0.5;
        model
    }

    #[test]
    fn strip_log_csv_has_headers() {
        let model = sample_model();
        let csv = model.to_strip_log_csv(None);
        assert!(csv.starts_with("TOP_LEVEL(m),BOTTOM_LEVEL(m),THICKNESS(m),REFERENCE"));
    }

    #[test]
    fn strip_log_svg_contains_elements() {
        let model = sample_model();
        let svg = model.render_strip_log_svg(StripLogRenderOptions::default());
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("stroke=\"#1E90FF\""));
    }

    #[test]
    fn ags_geol_csv_has_hole_id() {
        let model = sample_model();
        let csv = model.to_ags_geol_csv("BH101", None);
        assert!(csv.contains("\"BH101\""));
    }
}
