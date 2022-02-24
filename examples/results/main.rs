#!/usr/bin/env rust
#[allow(unused_variables)]

fn main(){
    use std::str::FromStr;
    // let mut results = vec![];
    // let mut errs = vec![];
    let nums:Vec<_> = vec!["17","1&","-27","ours","99"];
    print!("nums {:?}\n",nums);
    let filtered:Vec<_> = nums
    .into_iter()
    .map(u8::from_str)
    // .inspect(|x| results.push(x.clone()))
    // .inspect(|x| errs.extend(x.clone().err()))
    .flatten()
    .collect();
    print!("filtered {:?}\n",filtered);
    // use rustutils::types::type_of;
    // print!("filtered {:?}\n",type_of(filtered));
    print!("sum(filtered)={}\n",{let sum:u8 = filtered.iter().sum();sum});
    // println!("results{:?}",results);
    // println!("errs{:?}",errs);

    let e= vec![Ok(2),Ok(10),Err("err!"),Ok(4)];
    // collect for items
    let res:Result<Vec<_>,&str> = e.clone().into_iter().collect();
    assert_eq!(res,Err("err!"));
    let v= vec![Ok(2),Ok(10),Ok(4)];
    let res:Result<Vec<_>,&str> = v.clone().into_iter().collect();
    assert_eq!(res,Ok(vec![2,10,4]));
    // sum for items
    let res:Result<i32,&str> = e.clone().into_iter().sum();
    assert_eq!(res,Err("err!"));
    // sum for items
    let v= vec![Ok(2),Ok(10),Ok(4)];
    let res:Result<i32,&str> = v.clone().into_iter().sum();
    assert_eq!(res,Ok(2+10+4));
    // product for items
    let res:Result<i32,&str> = v.clone().into_iter().product();
    assert_eq!(res,Ok(2*10*4));

    // map for item
    // unwrap for item
    let res:Result<&str,bool> = Ok("Ok!");
    assert_eq!(res.unwrap(),"Ok!");
    let res:Result<i32,bool> = Ok(10).map(|i|i+1);
    assert_eq!(res.unwrap(),11);
    assert_eq!(res.is_ok(),true);
    // Ok(x)->Ok(y)
    let res:Result<i32,bool> = Ok(10).and_then(|i|Ok(i*2));
    assert_eq!(res.unwrap(),20);
    // Err
    // let res:Result<i32,&str> = Err("err!");
    // assert_eq!(res.unwrap(),"err!");   
    // Err->Ok
    let res:Result<i32,i32> = Err(10).or_else(|i|Ok(i*3));
    assert_eq!(res.unwrap(),30);
    assert_eq!(res.is_err(),false);
    
    // [digest]
    // unwrap vs. collect
    // collect returns err if err exists,
    // and all Oks as vec if no errs.
    // unwrap returns only Ok
    // collec for Vec<Result>, unwrap for Result

    fn test2(class:Option<u8>) -> Result<&'static str,bool> {
        match class {
            None => Err(false),
            Some(n @ 1..=3) => Ok("classied"),
            Some(4) => Ok("unclassifed"),
            Some(_) => Err(true),
            // Some(n) => Err(true),
        }
    }
    fn test(class:u8) -> Result<&'static str,bool> {
        match class {
            1 => Ok("business class"),
            2|3 => Ok("economy class"),
            4..=10 => Ok("no class"),
            _ => Err(true),
        }
    }
    assert_eq!(test(1).unwrap(),"business class");
    match test(11) {
        Ok(v) => println!("classifed into {}",v),
        Err(e) =>  println!("failure is {}",e),
    }
    assert_eq!(test2(Some(1)).unwrap(),"classied");
    match test2(Some(11)) {
        Ok(v) => println!("classifed into {}",v),
        Err(e) =>  println!("failure is {}",e),
    }
    let option:Option<u8> = Some(10);
    // let option:Option<u8> = None;
    assert_eq!(option.unwrap(),10);

    // [digest]
    // Some, None belongs to Option

    // [digest]
    // - Function returns Result
    // - Result leads to Decisions.
    // In other PLs, these two are not seperated. Input->Box->Output(result+handling)
    // but in rust, matching mechanism enhanced to become a function module.
}