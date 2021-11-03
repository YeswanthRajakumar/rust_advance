mod mean_median_mode;
mod pig_latin_conversion;

fn main() {
    mean_median_mode::get_result(vec![0,1,1,2,3,4,4,4,5,6]);
    pig_latin_conversion::get_result(&mut String::from("first"))

}