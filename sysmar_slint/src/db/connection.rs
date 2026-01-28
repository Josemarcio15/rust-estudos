use mysql::*;

pub fn get_connection() -> PooledConn{
    let url = "mysql://root:Amagedom12%40@127.0.0.1:3306/sys_db";

    let pool = Pool::new(url).expect("Erro ao criar pool");
    pool.get_conn().expect("Erro ao conectar no Mariadb")
}