use {
    crate::*,
    serde::Serialize,
    std::{
        fs::File,
        io::{self, Read},
        path::PathBuf,
    },
};

pub const DOLL_JS: &str = include_str!("../rsc/dom-doll.js");
pub const VIS_JS: &str = include_str!("../rsc/vis-timeline-graph2d.min.js");
pub const VIS_CSS: &str = include_str!("../rsc/vis-timeline-graph2d.min.css");
pub const SQL_JS: &str = include_str!("../rsc/sql-wasm.js");
pub const SQL_WASM: &[u8] = include_bytes!("../rsc/sql-wasm.wasm");
pub const VIEWER_JS: &str = include_str!("../rsc/viewer.js");
pub const VIEWER_CSS: &str = include_str!("../rsc/viewer.css");

#[derive(Debug, Serialize)]
struct Conf<'b> {
    bench_name: &'b str,
    task_name: Option<&'b str>,
    gb_version: String,
}

pub struct HtmlViewer<'b> {
    conf: Conf<'b>,
}

impl<'b> HtmlViewer<'b> {

    pub fn new(bench_name: &'b str, task_name: Option<&'b str>) -> Self {
        Self {
            conf: Conf {
                bench_name,
                task_name,
                gb_version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }

    pub fn open_in_browser(&self) -> Result<(), GlassBenchError> {
        let (mut w, path) = make_temp_file()?;
        self.write_html(&mut w)?;
        open::that(path)?;
        Ok(())
    }

    pub fn write_html<W: io::Write>(&self, mut w: W) -> Result<(), GlassBenchError> {
        writeln!(w, "<!DOCTYPE HTML>")?;
        writeln!(w, "<html>")?;
        writeln!(w, "<head>")?;
        writeln!(w, "<meta charset=UTF-8>")?;
        writeln!(w, "<style type=text/css>{}</style>", VIEWER_CSS)?;
        writeln!(w, "<style type=text/css>{}</style>", VIS_CSS)?;
        writeln!(w, "<script>{}</script>", VIS_JS)?;
        writeln!(w, "<script>{}</script>", SQL_JS)?;
        writeln!(w, "<script>{}</script>", DOLL_JS)?;
        writeln!(w, "<script charset=UTF-8>{}</script>", VIEWER_JS)?;
        write_db(&mut w)?;
        writeln!(w, "</head>")?;
        writeln!(w, "<body>")?;
        writeln!(w, "<script>")?;
        writeln!(w, "const gb_conf = {}", serde_json::to_string(&self.conf)?)?;
        writeln!(
            w,
            r#"const sql_conf = {{ locateFile: filename=>"data:application/wasm;base64,{}" }};"#,
            base64::encode(SQL_WASM),
        )?;
        writeln!(w, "main(sql_conf);")?;
        writeln!(w, "</script>")?;
        writeln!(w, "</body>")?;
        writeln!(w, "</html>")?;
        Ok(())
    }
}

pub fn make_temp_file() -> io::Result<(File, PathBuf)> {
    tempfile::Builder::new()
        .prefix("glassbench-")
        .suffix(".html")
        .rand_bytes(12)
        .tempfile()?
        .keep()
        .map_err(|_| io::Error::new(
            io::ErrorKind::Other,
            "temp file can't be kept",
        ))
}

pub fn write_db<W: io::Write>(mut w: W) -> Result<(), GlassBenchError> {
    let mut file = File::open(Db::path()?)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    writeln!(w,
        r#"<script>const db64="{}"</script>"#,
        base64::encode(&bytes),
    )?;
    Ok(())
}

