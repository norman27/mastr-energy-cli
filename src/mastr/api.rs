use std::fs;

pub(crate) fn fetch_json_cached(city: String) -> Option<String> {
    let data = fs::read_to_string(format!("./data/{}.json", city.to_lowercase()));

    match data {
        Ok(data) => Some(data),
        Err(_error) => fetch_json(city),
    }
}

fn fetch_json(city: String) -> Option<String> {
    let url = format!("https://www.marktstammdatenregister.de/MaStR/Einheit/EinheitJson/GetVerkleinerteOeffentlicheEinheitStromerzeugung?sort=EinheitMeldeDatum-desc&page=1&pageSize=10000&group=&filter=Ort~eq~%27{}%27", city);

    let body = reqwest::blocking::get(url).ok()?.text().ok()?;

    //@TODO might want to cache this

    Some(body)
}
