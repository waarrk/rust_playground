#!/usr/bin/env cargo +nightly -Z script

//! ```cargo
//! [dependencies]
//! plotters = "0.3.3"
//! ```

use plotters::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ディレクトリが存在しない場合は作成
    fs::create_dir_all("plotters-output")?;

    // 描画エリアの初期化
    let root = BitMapBackend::new("plotters-output/sample.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // 図枠/グリッド設定
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;
    // 図枠/グリッド描画
    chart.configure_mesh().draw()?;

    // プロット描画
    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // ラベルの設定と描画
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // 描画結果保存
    root.present()?;

    Ok(())
}
