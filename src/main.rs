use charming::{
    component::{Axis, Grid, Legend, Title},
    element::{AxisPointer, AxisType},
    series::Line,
    Chart, HtmlRenderer,
};
use data::{
    means_of_transport::MeansOfTransport, periods::Periods, typed_dataset::TypedDataset, urls::Urls,
};
use std::collections::HashMap;

pub mod data;

const MAIN_URL: &str = "https://opendata.cbs.nl/ODataApi/OData/85347ENG";

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let urls = reqwest::get(MAIN_URL).await?.json::<Urls>().await?;

    let typed_dataset = reqwest::get(
        &urls
            .data
            .iter()
            .find(|v| v.name == "TypedDataSet")
            .unwrap()
            .url,
    )
    .await?
    .json::<TypedDataset>()
    .await?;

    let periods = reqwest::get(&urls.data.iter().find(|v| v.name == "Periods").unwrap().url)
        .await?
        .json::<Periods>()
        .await?;

    let means_of_transport = reqwest::get(
        &urls
            .data
            .iter()
            .find(|v| v.name == "MeansOfTransport")
            .unwrap()
            .url,
    )
    .await?
    .json::<MeansOfTransport>()
    .await?;

    let mut categories = HashMap::new();

    for value in typed_dataset.data {
        categories
            .entry(value.means_of_transport)
            .or_insert(vec![])
            .push(value.carbon_dioxide_co2_1);
    }

    let mut chart = Chart::new()
        .title(Title::new().text("Emissions to air on Dutch territory; road traffic"))
        .legend(Legend::new().top("bottom"))
        .grid(Grid::new().show(true))
        .axis_pointer(AxisPointer::new().show(true).snap(false))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .name("Year")
                .data(periods.data.iter().map(|v| &v.title).collect()),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Carbon dioxide (million kg)"),
        );

    let mut categories = categories.into_iter().collect::<Vec<_>>();

    categories.sort_by(|a, b| a.0.cmp(&b.0));

    for (vehicle, carbon_dioxide_co2_1) in categories {
        chart = chart.series(
            Line::new()
                .name(
                    &means_of_transport
                        .data
                        .iter()
                        .find(|v| v.key == vehicle)
                        .unwrap()
                        .title,
                )
                .data(carbon_dioxide_co2_1),
        );
    }

    let mut renderer = HtmlRenderer::new("Title", 800, 600);
    renderer.save(&chart, "emission_data.html")?;

    Ok(())
}
