fn add_space(mut s: String, mut n:i32)->String{
while n>=0{
    s.push_str(" ");
    n=n-1;
}
return s;
}
fn add_str(mut s:String, str:&str)->String{
    s.push_str(&str);
    return s;
    }
fn oglindit(mut n:i32)->i32{
    let mut og:i32=0;
    while n>0{
        og=og*10+n%10;
        n=n/10;
    }
    return og;
}
fn add_integer(mut s:String,n:i32)->String{
    let mut t = String::from("");
    let mut m:i32=oglindit(n);
    let mut i=0;
    while m>0{
        let a:i32;
        a=m%10;
        let c:char=(a as u8+ b'0')as char;
        if i%3==0&&i!=0
            { t.push('_');
             t.push(c);
            i+=1;}
        else {
            t.push(c);
            i+=1;
        }
       
        m=m/10;
    }

    s.push_str(&t);
    return s;

}

fn add_float(mut s:String,n:f32)->String{
    let mut t = String::from("");
    let  m:i32;
    m=n as i32;
    let mut x=oglindit(m);
    let y=m as f32;

    while x>0{
        let a:i32;
        a=x%10;
        let c:char=(a as u8+ b'0')as char;
        t.push(c);
        x=x/10;
    }
    s.push_str(&t);
    s.push(',');
    let mut b:i32;
    let mut z=n-y;
    while z!=0.0
    {
        z=z*10.0;
        b=z as i32;
        let q=b as f32;
        let c:char=(b as u8+ b'0')as char;
        s.push(c);
        z=z-q;
    }

    

    return s;

}

fn main() {
    let mut text = String::new();

    text = add_space(text, 50);
    text = add_str(text, "I 💚");
    text.push_str("\n");
    text = add_space(text, 50);
    text = add_str(text, "RUST.");
    text.push_str("\n");
    text.push_str("\n");
    text = add_space(text, 5);
    text = add_str(text, "Most");
    text = add_space(text, 11);
    text = add_str(text, "create");
    text = add_space(text, 6);
    text = add_integer(text, 306437968);
    text = add_space(text, 14);
    text = add_str(text, "and");
    text = add_space(text, 8);
    text = add_str(text, "lastest");
    text = add_space(text, 9);
    text = add_str(text, "is");
    text.push_str("\n");
    text = add_space(text, 10);
    text = add_str(text, "downloaded");
    text = add_space(text, 8);
    text = add_str(text, "has");
    text = add_space(text, 13);
    text = add_str(text, "downloads");
    text = add_space(text, 9);
    text = add_str(text, "the");
    text = add_space(text, 11);
    text = add_str(text, "version");
    text = add_space(text, 4);
    text = add_float(text, 2.039);
    text = add_str(text, ".");

    println!("{}", text);


}
