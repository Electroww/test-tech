
mod get_opendatasoft_data;
mod count_activite_principale_etablissement;

fn main() {
    let count ;
    if let Ok(data) = get_opendatasoft_data::get_opendatasoft_data() {
        count = count_activite_principale_etablissement::count(&data);
        println!("{:?}", count);
    } else {
        println!("Error: unable to fetch data from opendatasoft");
    }
}