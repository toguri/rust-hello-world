fn main() {
    {
        let x = String::from("hello1");
        let y = x;

        //println!("{}", x);
        println!("{}", y);
    }
    {
        let z = String::from("hello2");
        {
            let w = &z;

            //let s = z;
            println!("w:{}:", w);
        }
        println!("z:{}:", z);
    }    
}
