
fn main() {

    let arr:[u32;5]=[1,2,3,4,7];

    fn print_arr(arr:[u32;5]){
        for i in &arr{
            println!("{}",i);
        }
    }

    print_arr(arr)



}
