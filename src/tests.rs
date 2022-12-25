


use crate::simulator::*;




pub fn test(){

    // # TEST BASIC:
    let map_s = vec!["00 00 00 00 00 00 00".to_string(),
                                "00 00 00 00 00 00 00".to_string(),
                                "00 00 00 00 00 00 00".to_string(),
                                "00 Sr_gr 05 05 05 E0010_gr 00".to_string(),
                                "00 00 00 00 00 00 00".to_string(),
                                "00 00 00 00 00 00 00".to_string(),
                                "00 00 00 00 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res:  (WON, steps) {:?}", res);
    assert! (res.0 == true);


    // # TEST SCAMBIO:
    let map_s = vec!["00 00 00 00 00 00 00".to_string(),
                                "00 00 00 00 00 00 00".to_string(),
                                "00 00 00 00 00 00 00".to_string(),
                                "00 Sr_grgr 05 45 05 E0010_rr 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 E1000_gg 00 00 00".to_string(),
                                "00 00 00 00 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res: (WON, steps) {:?}", res);
    assert! (res.0 == true);


    // # TEST CENTER CROSSING:
    let map_s = vec!["00 00 00 E0100_go 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "Sr_br 05 05 25 05 05 E0010_go".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 St_yy 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res:  (WON, steps) {:?}", res);
    assert! (res.0 == true);
    

    // # TEST CENTER COLLIDING:
    let map_s = vec!["00 00 00 E0100_g 00 00 00".to_string(),
                                "00 00 06 01 00 00 00".to_string(),
                                "00 00 63 04 00 00 00".to_string(),
                                "Sr_b 05 01 02 06 05 E0010_g".to_string(),
                                "00 00 00 23 01 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 St_y 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res:  (WON, steps) {:?}", res);
    assert! (res.0 == true);


    // # TEST BORDER COLLIDING:
    let map_s = vec!["00 00 00 E0100_g 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 06 01 00 00 00".to_string(),
                                "Sr_b 05 53 45 05 05 E0010_g".to_string(),
                                "00 00 00 23 00 00 00".to_string(),
                                "00 00 00 St_y 00 00 00".to_string(),
                                "00 00 00 00 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res:  (WON, steps) {:?}", res);
    assert! (res.0 == true);


    // # TEST MERGING COLLIDING:
    let map_s = vec!["00 00 00 E0100_g 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "Sr_b 05 05 21 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 St_y 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res:  (WON, steps) {:?}", res);
    assert! (res.0 == true);
    
  
    // # TEST MERGING COLLIDING AT THE LAST MOMENT:
    let map_s = vec!["00 00 00 E0100_g 00 00 00".to_string(),
                                "00 06 05 12 00 00 00".to_string(),
                                "00 02 00 02 00 00 00".to_string(),
                                "Sr_b 01 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 St_y 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res:  (WON, steps) {:?}", res);
    assert! (res.0 == true);


    // # TEST SPLITTING:
    let map_s = vec!["00 00 00 E0100_b 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "Sr_g 51 05 D3 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 E1000_y 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res:  (WON, steps) {:?}", res);
    assert! (res.0 == true);


    // # TEST COLORING:
    let map_s = vec!["00 00 00 E0100_g 00 00 00".to_string(),
                                "00 00 00 g2 00 00 00".to_string(),
                                "00 06 05 12 00 00 00".to_string(),
                                "Sr_b 01 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 02 00 00 00".to_string(),
                                "00 00 00 St_o 00 00 00".to_string()];
    let map = parseMap(map_s);
    prettyPrintMap(&map);
    let res = runLevel(map, true, true, 20);
    println!("res:  (WON, steps) {:?}", res);

                                

}


















