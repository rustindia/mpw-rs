extern crate rusqlite;

use self::rusqlite::Connection;
use arg_parse::MpwOptions;
use self::rusqlite::types::{FromSql, FromSqlResult, ValueRef, ToSql, ToSqlOutput, FromSqlError};
use self::rusqlite::{Result};
use common::{SiteVariant, SiteType};

pub fn saveToSqlLite(mpw_options: MpwOptions) -> MpwOptions {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("CREATE TABLE mpwOptions (\
                    site TEXT NOT NULL,\
                    user TEXT NOT NULL,\
                    variant INTEGER,\
                    template INTEGER,\
                    counter INTEGER,\
                    algo TEXT NOT NULL,\
                    context TEXT NOT NULL\
    )",&[]).unwrap();

    let variant = mpw_options.variant as i32;
    let template = mpw_options.template as i32;
    let counter = mpw_options.counter as i32;

    println!("checking");

    let does_exist: Result<i32> = conn.query_row("SELECT counter FROM mpwOptions \
            WHERE site = ? and user = ? and variant = ? and template = ?\
            and algo = ? and context = ?", &[&mpw_options.site, &mpw_options.user, &variant, &template,
        &mpw_options.algo, &mpw_options.context], |r| r.get(0));

    let test:bool = match does_exist{
        Ok(count) => true,
        _ => false,
    };
    println!("testing {}",test);
    
    if !test {
        println!("doesnt exist");
        conn.execute("INSERT INTO mpwOptions \
                (site,user,variant,template,counter,algo,context)\
                VALUES (?1,?2,?3,?4,?5,?6,?7)",
                     &[&mpw_options.site, &mpw_options.user, &variant, &template,
                         &counter, &mpw_options.algo, &mpw_options.context]);
    }
    else {
        println!("does exist");
        let updated_counter = counter + 1;
        conn.execute("UPDATE mpwOptions SET counter = ?\
                WHERE site = ? and user = ? and variant = ? and template = ?\
    and algo = ? and context = ?",
                     &[&updated_counter, &mpw_options.site, &mpw_options.user, &variant, &template,
                         &mpw_options.algo, &mpw_options.context]);
    }
//
//    let mut out:i32 = conn.query_row("SELECT counter FROM mpwOptions \
//    WHERE site = ? and user = ? and variant = ? and template = ?\
//    and algo = ? and context = ?",&[&mpw_options.site, &mpw_options.user, &variant, &template,
//        &mpw_options.algo, &mpw_options.context], |r| r.get(0)).unwrap();
//
//    impl FromSql for SiteVariant {
//        fn column_result(value: ValueRef) -> FromSqlResult<SiteVariant> {
//            Ok(SiteVariant::from(value.as_str().unwrap()).unwrap())
//        }
//    }
//
//    impl FromSql for SiteType {
//        fn column_result(value: ValueRef) -> FromSqlResult<SiteType> {
//            Ok(SiteType::from(value.as_str().unwrap()).unwrap())
//        }
//    }
//
//    println!("input counter {:?}",out);
//    let results: Result<Vec<String>> = out.query_map(&[], |row| row.get(1))
//        .unwrap()
//        .collect();
//    println!("results {:?}",results);
//    let out_iter: Result<Vec<MpwOptions>> = out.query_map(&[], |eachRow|{
//        MpwOptions{
//            site: eachRow.get(0),
//            user: eachRow.get(1),
//            variant: eachRow.get(2),
//            template: eachRow.get(3),
//            counter: eachRow.get(4),
//            algo: eachRow.get(5),
//            context: eachRow.get(6),
//        }
//    }).unwrap().collect();

//    let out_iter: Result<Vec<String>> = out.query_map(&[], |eachRow| eachRow.get(1)).unwrap().collect();
//    println!("out_iter {:?}",out_iter);

//    for eachMpwOptions in out_iter {
//        println!("The struct to be saved is {:?}", eachMpwOptions.unwrap());
//    }
//
    MpwOptions{
        site: "test".to_string(),
        user: "test".to_string(),
        variant: SiteVariant::Password,
        template: SiteType::Maximum,
        counter: 1,
        algo: "0".to_string(),
        context: "test".to_string(),
    }
}