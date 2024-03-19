pub mod intervalo;
pub mod iteracion;

use iteracion::Iteracion;

use self::intervalo::Intervalo;

pub fn falsa_posicion(
    intervalo: Intervalo,
    funcion: impl Fn(f64) -> f64,
    precision: f64,
) -> Vec<Iteracion> {
    let mut iteraciones: Vec<Iteracion> = vec![];
    let mut iteracion = Iteracion {
        intervalo,
        numero: 0,
        aproximacion: intervalo.0,
        error: 100.0,
    };
    loop {
        let sec = formula_secante(&iteracion.intervalo, &funcion);
        let intervalo_sig;

        if funcion(iteracion.intervalo.0) * funcion(sec) < 0.0 {
            intervalo_sig = Intervalo(iteracion.intervalo.0, sec);
        } else {
            intervalo_sig = Intervalo(sec, iteracion.intervalo.1);
        }

        let nueva_iteracion = Iteracion {
            intervalo: intervalo_sig,
            numero: iteracion.numero + 1,
            aproximacion: sec,
            error: error_abs(sec, iteracion.aproximacion),
        };

        if error_abs(sec, iteracion.aproximacion) > precision {
            iteracion = nueva_iteracion;
            iteraciones.push(nueva_iteracion);
        } else {
            iteraciones.push(nueva_iteracion);
            break;
        }
    }

    iteraciones
}

pub fn error_abs(pn: f64, pn_1: f64) -> f64 {
    (pn - pn_1).abs()
}

pub fn formula_secante(intervalo: &Intervalo, funcion: &impl Fn(f64) -> f64) -> f64 {
    let numerador = funcion(intervalo.1) * (intervalo.1 - intervalo.0);
    let denominador = funcion(intervalo.1) - funcion(intervalo.0);

    intervalo.1 - (numerador / denominador)
}
