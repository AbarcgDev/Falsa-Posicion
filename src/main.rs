mod metodo_falsa_posicion;

use metodo_falsa_posicion::{falsa_posicion, intervalo::Intervalo};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Funcion a resolver
    #[arg(short, long, required = true)]
    funcion: String,

    /// Variable en la que se expresa la funcion
    #[arg(short, long, default_value_t = String::from("x"))]
    variable: String,

    ///Valor menor del intervalo en el cual buscar raiz
    #[arg(short, long, required = true)]
    a: f64,

    ///Valor mayor del intervalo en el cual buscar raiz
    #[arg(short, long, required = true)]
    b: f64,

    /// Precision requerida
    #[arg(short, long, default_value_t = 10e-10)]
    precision: f64,
}

fn main() {
    let args = Args::parse();

    let intervalo = Intervalo(args.a, args.b);
    let expr: meval::Expr = args.funcion.parse().unwrap();
    let func = expr.bind(args.variable.as_str()).unwrap();

    if func(args.a) * func(args.b) > 0.0 {
        eprintln!("El intervalo proporcionado no satisface la condicion f(a) * f(b) < 0");
        std::process::exit(1);
    }

    let iteraciones = falsa_posicion(intervalo, func, args.precision);
    for iter in &iteraciones {
        println!(
            "Iteracion: {numero:}\n\tAproximacion: {aproximacion}\n\tError: {error}",
            numero = iter.numero,
            aproximacion = iter.aproximacion,
            error = iter.error
        );
    }
    println!(
        "La raiz solicitada es: {raiz:}",
        raiz = iteraciones.get(iteraciones.len() - 1).unwrap().aproximacion
    )
}
