extern crate rusqlite;

use self::rusqlite::Connection;
use arg_parse::MpwOptions;
use self::rusqlite::{Result};


pub fn save_to_sql_lite(mpw_options: MpwOptions) -> MpwOptions {
    let conn = Connection::open("site_counter.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS mpwOptions (\
                    site TEXT NOT NULL,\
                    user TEXT NOT NULL,\
                    variant INTEGER,\
                    template INTEGER,\
                    counter INTEGER,\
                    algo TEXT NOT NULL,\
                    context TEXT NOT NULL\
    )", &[]).unwrap();

    let variant = mpw_options.variant.clone() as i32;
    let template = mpw_options.template.clone() as i32;
    let counter = mpw_options.counter as i32;
    let mut updated_counter = counter;

    let does_exist: Result<i32> = conn.query_row("SELECT counter FROM mpwOptions \
            WHERE site = ? and user = ? and variant = ? and template = ?\
            and algo = ? and context = ?", &[&mpw_options.site, &mpw_options.user, &variant, &template,
        &mpw_options.algo, &mpw_options.context], |r| r.get(0));

    let test: bool = match does_exist {
        Ok(_) => true,
        _ => false,
    };

    if !test {
        conn.execute("INSERT INTO mpwOptions \
                (site,user,variant,template,counter,algo,context)\
                VALUES (?1,?2,?3,?4,?5,?6,?7)",
                     &[&mpw_options.site, &mpw_options.user, &variant, &template,
                         &counter, &mpw_options.algo, &mpw_options.context]).unwrap();
    } else {
        updated_counter = does_exist.unwrap() + 1;
        println!("Current Site Counter is: {}",updated_counter);
        conn.execute("UPDATE mpwOptions SET counter = ?\
                WHERE site = ? and user = ? \
                and variant = ? and template = ? \
                and algo = ? and context = ?",
                     &[&updated_counter, &mpw_options.site, &mpw_options.user, &variant, &template,
                         &mpw_options.algo, &mpw_options.context]).unwrap();
    }

    MpwOptions {
        site: mpw_options.site,
        user: mpw_options.user,
        variant: mpw_options.variant,
        template: mpw_options.template,
        counter: updated_counter,
        algo: mpw_options.algo,
        context: mpw_options.context,
    }
}