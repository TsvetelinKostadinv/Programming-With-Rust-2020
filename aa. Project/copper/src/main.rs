fn print_ascii_art()
{
    println!( 
r#"
_________                                    
\_   ___ \  ____ ______ ______   ___________ 
/    \  \/ /  _ \\____ \\____ \_/ __ \_  __ \
\     \___(  <_> )  |_> >  |_> >  ___/|  | \/
 \______  /\____/|   __/|   __/ \___  >__|   
        \/       |__|   |__|        \/       
        "#);
}

fn main() {
    print_ascii_art();
    println!("Hello, welcome to copper!");
}
