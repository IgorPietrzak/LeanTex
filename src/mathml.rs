use crate::lean::LeanParsed;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

#[derive(Debug)]
pub struct Markdown {
    file_content: String,
}

impl Markdown {
    pub fn new(parsed_lean: LeanParsed) -> Self {
        let content = make_content(parsed_lean);

        Self {
            file_content: content,
        }
    }

    pub fn create_file(&self, file_name: &str) -> Result<(), Error> {
        let mut file = File::create(format!("{}.html", file_name))?;
        file.write_all(self.file_content.as_bytes())?;
        Ok(())
    }
}

fn make_content(parsed_lean: LeanParsed) -> String {
    let content = format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Lean</title>
    <style>
      body {{
        background-color: black;
        color: hotpink;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
      }}
    </style>
  </head>
  <body>
    <h1>Theorem Name</h1>
    <p>Theorem documentation maybe</p>
    <math xmlns="http://www.w3.org/1998/Math/MathML">
      <mrow>
        <mi>x</mi>
        <mo>=</mo>
        <mfrac>
          <mrow>
            <mo>-</mo>
            <mi>b</mi>
            <mo>&#xB1;</mo>
            <!-- Unicode for ± -->
            <msqrt>
              <mrow>
                <msup>
                  <mi>b</mi>
                  <mn>2</mn>
                </msup>
                <mo>-</mo>
                <mn>4</mn>
                <mo>&#x2062;</mo>
                <!-- Invisible times -->
                <mi>a</mi>
                <mo>&#x2062;</mo>
                <!-- Invisible times -->
                <mi>c</mi>
              </mrow>
            </msqrt>
          </mrow>
          <mrow>
            <mn>2</mn>
            <mo>&#x2062;</mo>
            <!-- Invisible times -->
            <mi>a</mi>
          </mrow>
        </mfrac>
      </mrow>
    </math>
    <p>
    {:?}
    </p>
  </body>
</html>"#,
        parsed_lean
    );
    content
}
