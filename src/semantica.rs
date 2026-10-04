#[derive(Clone, Copy, PartialEq)]
pub enum Tipo {
    Heartbeat,
    Condicional,
    Gauge,
    Estatistico,
    Contador,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Gravidade {
    Baixa,
    Media,
    Alta,
    Critica,
}

pub struct Contador {
    pub indice: usize,
    pub nome: &'static str,
    pub modulo: &'static str,
    pub tipo: Tipo,
    pub quando_sobe: &'static str,
    pub limiar: &'static str,
    pub gravidade: Gravidade,
}

pub const CONTADORES: [Contador; 18] = [
    Contador { indice: 0, nome: "phl_processos", modulo: "PHL", tipo: Tipo::Gauge, quando_sobe: "sempre", limiar: "nenhum", gravidade: Gravidade::Baixa },
    Contador { indice: 1, nome: "guardian_ciclos", modulo: "Guardiao", tipo: Tipo::Heartbeat, quando_sobe: "cada ciclo", limiar: "nenhum", gravidade: Gravidade::Baixa },
    Contador { indice: 2, nome: "guardian_isolamentos", modulo: "Guardiao", tipo: Tipo::Condicional, quando_sobe: "processos > 50", limiar: "50 processos", gravidade: Gravidade::Media },
    Contador { indice: 3, nome: "zeus_ciclos", modulo: "Zeus", tipo: Tipo::Heartbeat, quando_sobe: "cada ciclo", limiar: "nenhum", gravidade: Gravidade::Baixa },
    Contador { indice: 4, nome: "aguia_alertas", modulo: "Aguia", tipo: Tipo::Condicional, quando_sobe: "ram > 65%", limiar: "65%", gravidade: Gravidade::Alta },
    Contador { indice: 5, nome: "aguia_nivel", modulo: "Aguia", tipo: Tipo::Gauge, quando_sobe: "sempre", limiar: "0 1 2", gravidade: Gravidade::Baixa },
    Contador { indice: 6, nome: "halyn_ciclos", modulo: "Halyn", tipo: Tipo::Heartbeat, quando_sobe: "cada ciclo", limiar: "nenhum", gravidade: Gravidade::Baixa },
    Contador { indice: 7, nome: "aegis_ciclos", modulo: "Aegis", tipo: Tipo::Heartbeat, quando_sobe: "cada ciclo", limiar: "nenhum", gravidade: Gravidade::Baixa },
    Contador { indice: 8, nome: "aegis_bloqueios", modulo: "Aegis", tipo: Tipo::Condicional, quando_sobe: "requisicao suspeita", limiar: "nenhum", gravidade: Gravidade::Alta },
    Contador { indice: 9, nome: "maqso_adaptacoes", modulo: "Maqso", tipo: Tipo::Condicional, quando_sobe: "score < 20", limiar: "20", gravidade: Gravidade::Alta },
    Contador { indice: 10, nome: "maqso_score", modulo: "Maqso", tipo: Tipo::Gauge, quando_sobe: "sempre", limiar: "0 a 100", gravidade: Gravidade::Baixa },
    Contador { indice: 11, nome: "desvios_detectados", modulo: "Detector", tipo: Tipo::Estatistico, quando_sobe: "desvio > 3 sigma", limiar: "3 sigma", gravidade: Gravidade::Media },
    Contador { indice: 12, nome: "ciclos_normais", modulo: "Detector", tipo: Tipo::Estatistico, quando_sobe: "sem desvio", limiar: "nenhum", gravidade: Gravidade::Baixa },
    Contador { indice: 13, nome: "reinicios", modulo: "Watchdog", tipo: Tipo::Condicional, quando_sobe: "batimento parado", limiar: "2 segundos", gravidade: Gravidade::Critica },
    Contador { indice: 14, nome: "ciclos_totais", modulo: "Motor", tipo: Tipo::Heartbeat, quando_sobe: "cada ciclo", limiar: "nenhum", gravidade: Gravidade::Baixa },
    Contador { indice: 15, nome: "persistencias", modulo: "Motor", tipo: Tipo::Contador, quando_sobe: "estado gravado", limiar: "60 segundos", gravidade: Gravidade::Baixa },
    Contador { indice: 16, nome: "tempo_ultimo_ciclo", modulo: "Motor", tipo: Tipo::Gauge, quando_sobe: "cada ciclo", limiar: "nenhum", gravidade: Gravidade::Baixa },
    Contador { indice: 17, nome: "batimento", modulo: "Motor", tipo: Tipo::Heartbeat, quando_sobe: "cada ciclo", limiar: "nenhum", gravidade: Gravidade::Critica },
];

pub fn nome(indice: usize) -> &'static str {
    CONTADORES[indice].nome
}

pub fn modulo(indice: usize) -> &'static str {
    CONTADORES[indice].modulo
}

pub fn gravidade(indice: usize) -> Gravidade {
    CONTADORES[indice].gravidade
}

pub fn descrever(indice: usize, delta: f64, media: f64, sigma: f64) -> String {
    let c = &CONTADORES[indice];
    let desvio = if sigma > 0.0 { (delta - media).abs() / sigma } else { 0.0 };
    format!(
        "contador={} modulo={} valor_delta={:.0} media={:.2} sigma={:.2} desvio={:.2} quando={} limiar={} gravidade={:?}",
        c.nome, c.modulo, delta, media, sigma, desvio, c.quando_sobe, c.limiar, c.gravidade
    )
}
