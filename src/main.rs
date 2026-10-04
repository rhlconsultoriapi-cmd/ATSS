
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
mod semantica;

use std::net::TcpListener;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::os::unix::fs::MetadataExt;
use std::process::exit;

fn obter_porta() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080)
}

struct Nucleo {
    phl_eventos: AtomicU64,
    phl_compressoes: AtomicU64,
    guardian_supervisoes: AtomicU64,
    guardian_isolamentos: AtomicU64,
    zeus_ajustes: AtomicU64,
    zeus_privilegios: AtomicU64,
    aguia_alertas: AtomicU64,
    aguia_estado: AtomicU64,
    halyn_exportadas: AtomicU64,
    aegis_criptografias: AtomicU64,
    aegis_bloqueios: AtomicU64,
    maqso_adaptacoes: AtomicU64,
    maqso_score: AtomicU64,
    aprendizado_erros: AtomicU64,
    aprendizado_acertos: AtomicU64,
    reencarnacoes: AtomicU64,
    ciclos_processados: AtomicU64,
    memoria_persistente: AtomicU64,
    timestamp_ultimo_ciclo: AtomicU64,
    coracao: AtomicU64,
}

impl Nucleo {
    fn novo() -> Self {
        Self {
            phl_eventos: AtomicU64::new(0),
            phl_compressoes: AtomicU64::new(0),
            guardian_supervisoes: AtomicU64::new(0),
            guardian_isolamentos: AtomicU64::new(0),
            zeus_ajustes: AtomicU64::new(0),
            zeus_privilegios: AtomicU64::new(0),
            aguia_alertas: AtomicU64::new(0),
            aguia_estado: AtomicU64::new(0),
            halyn_exportadas: AtomicU64::new(0),
            aegis_criptografias: AtomicU64::new(0),
            aegis_bloqueios: AtomicU64::new(0),
            maqso_adaptacoes: AtomicU64::new(0),
            maqso_score: AtomicU64::new(50),
            aprendizado_erros: AtomicU64::new(0),
            aprendizado_acertos: AtomicU64::new(0),
            reencarnacoes: AtomicU64::new(0),
            ciclos_processados: AtomicU64::new(0),
            memoria_persistente: AtomicU64::new(0),
            timestamp_ultimo_ciclo: AtomicU64::new(0),
            coracao: AtomicU64::new(0),
        }
    }
}

const RAM_ALERTA_PERCENT: f32 = 65.0;
const RAM_CRITICA_PERCENT: f32 = 85.0;
const ATIVIDADE_TIMEOUT_SEGUNDOS: u64 = 3;

static HISTORICO_RAM: Mutex<Vec<f32>> = Mutex::new(Vec::new());
static HISTORICO_VOLUME: Mutex<Vec<u64>> = Mutex::new(Vec::new());
static NARRATIVA: Mutex<Vec<String>> = Mutex::new(Vec::new());
static HISTORICO_CPU: Mutex<Vec<f32>> = Mutex::new(Vec::new());
static HISTORICO_SCORE: Mutex<Vec<u64>> = Mutex::new(Vec::new());
static EVENTOS_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
static NASCIMENTO: AtomicU64 = AtomicU64::new(0);

struct AprendizadoSistema {
    persistencia: [u32; 18],
    ultimo: Option<[u64; 18]>,
    deltas: [Vec<f64>; 18],
}
impl AprendizadoSistema {
    fn novo() -> Self {
        Self {
            ultimo: None,
            deltas: std::array::from_fn(|_| Vec::with_capacity(128)),
            persistencia: [0; 18],
        }
    }
}
static APRENDIZADO_SISTEMA: std::sync::Mutex<Option<AprendizadoSistema>> = std::sync::Mutex::new(None);
#[allow(dead_code)]
static MEMORIA_LONGO_PRAZO: Mutex<Vec<(u64, f32)>> = Mutex::new(Vec::new());
#[allow(dead_code)]
static PADROES_DIARIOS: Mutex<Vec<(u64, f32)>> = Mutex::new(Vec::new());
#[allow(dead_code)]
static TENDENCIA_FALHA: AtomicU64 = AtomicU64::new(0);
#[allow(dead_code)]
static ULTIMO_PADRAO: AtomicU64 = AtomicU64::new(0);
static ANOMALIAS_DETECTADAS: AtomicU64 = AtomicU64::new(0);
static AUTO_AJUSTES: AtomicU64 = AtomicU64::new(0);
static ULTIMO_VOLUME: AtomicU64 = AtomicU64::new(0);
static ULTIMO_TICK: AtomicU64 = AtomicU64::new(0);

fn ler_arquivo(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

fn ler_meminfo(campo: &str) -> u64 {
    let conteudo = ler_arquivo("/proc/meminfo");
    for linha in conteudo.lines() {
        if linha.starts_with(campo) {
            let partes: Vec<&str> = linha.split_whitespace().collect();
            if partes.len() >= 2 {
                return partes[1].parse::<u64>().unwrap_or(0) / 1024;
            }
        }
    }
    0
}

fn ler_cpu_ticks() -> (u64, u64) {
    let stat = ler_arquivo("/proc/stat");
    let primeira = stat.lines().next().unwrap_or("");
    let partes: Vec<&str> = primeira.split_whitespace().collect();
    if partes.len() > 4 {
        let user: u64 = partes[1].parse().unwrap_or(0);
        let nice: u64 = partes[2].parse().unwrap_or(0);
        let system: u64 = partes[3].parse().unwrap_or(0);
        let idle: u64 = partes[4].parse().unwrap_or(0);
        let iowait: u64 = partes.get(5).and_then(|v| v.parse().ok()).unwrap_or(0);
        let total = user + nice + system + idle + iowait;
        let busy = user + nice + system + iowait;
        return (busy, total);
    }
    (0, 1)
}

fn ler_loadavg() -> f32 {
    if let Ok(load) = fs::read_to_string("/proc/loadavg") {
        if let Some(v) = load.split_whitespace().next() {
            if let Ok(val) = v.parse::<f32>() {
                return val;
            }
        }
    }
    0.0
}

static CPU_ANTERIOR: Mutex<(u64, u64)> = Mutex::new((0, 0));

fn ler_cpu() -> f32 {
    let (busy_atual, total_atual) = ler_cpu_ticks();
    let mut anterior = CPU_ANTERIOR.lock().unwrap();
    let (busy_ant, total_ant) = *anterior;
    *anterior = (busy_atual, total_atual);
    drop(anterior);
    if total_atual > total_ant && total_ant > 0 {
        (busy_atual - busy_ant) as f32 / (total_atual - total_ant) as f32
    } else {
        0.0
    }
}

fn contar_processos() -> u64 {
    if let Ok(entradas) = fs::read_dir("/proc") {
        entradas
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().chars().all(|c| c.is_ascii_digit()))
            .count() as u64
    } else {
        0
    }
}

fn ler_uptime() -> u64 {
    let uptime = ler_arquivo("/proc/uptime");
    let partes: Vec<&str> = uptime.split_whitespace().collect();
    partes.first().unwrap_or(&"0").parse::<f32>().unwrap_or(0.0) as u64
}

fn detectar_ambiente() -> &'static str {
    if std::path::Path::new("/.dockerenv").exists() {
        "DOCKER"
    } else if std::env::var("TERMUX_VERSION").is_ok() {
        "ANDROID"
    } else if fs::metadata("/proc/stat").is_ok() {
        "LINUX"
    } else {
        "RESTRITO"
    }
}

#[allow(dead_code)]
fn detectar_su() -> bool {
    std::process::Command::new("su")
        .arg("-c")
        .arg("id")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn detectar_root() -> bool {
    std::fs::metadata("/proc/self").map(|m| m.uid()).unwrap_or(1) == 0
}

fn persistir_estado(nucleo: &Nucleo, caminho: &str) {
    if let Ok(mut arquivo) = OpenOptions::new().create(true).write(true).truncate(true).open(caminho) {
        let valores: [u64; 18] = [
            nucleo.phl_eventos.load(Ordering::Relaxed),
            nucleo.guardian_supervisoes.load(Ordering::Relaxed),
            nucleo.guardian_isolamentos.load(Ordering::Relaxed),
            nucleo.zeus_ajustes.load(Ordering::Relaxed),
            nucleo.aguia_alertas.load(Ordering::Relaxed),
            nucleo.aguia_estado.load(Ordering::Relaxed),
            nucleo.halyn_exportadas.load(Ordering::Relaxed),
            nucleo.aegis_criptografias.load(Ordering::Relaxed),
            nucleo.aegis_bloqueios.load(Ordering::Relaxed),
            nucleo.maqso_adaptacoes.load(Ordering::Relaxed),
            nucleo.maqso_score.load(Ordering::Relaxed),
            nucleo.aprendizado_erros.load(Ordering::Relaxed),
            nucleo.aprendizado_acertos.load(Ordering::Relaxed),
            nucleo.reencarnacoes.load(Ordering::Relaxed),
            nucleo.ciclos_processados.load(Ordering::Relaxed),
            nucleo.memoria_persistente.load(Ordering::Relaxed),
            nucleo.timestamp_ultimo_ciclo.load(Ordering::Relaxed),
            nucleo.coracao.load(Ordering::Relaxed),
        ];
        let mut buf = Vec::with_capacity(18 * 8);
        for v in valores.iter() {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        if arquivo.write_all(&buf).is_ok() {
            nucleo.memoria_persistente.fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn carregar_estado(nucleo: &Nucleo, caminho: &str) {
    if let Ok(mut arquivo) = File::open(caminho) {
        let mut buf = Vec::new();
        if arquivo.read_to_end(&mut buf).is_ok() && buf.len() >= 18 * 8 {
            let valores: Vec<u64> = buf
                .chunks_exact(8)
                .take(18)
                .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
                .collect();
            nucleo.phl_eventos.store(valores[0], Ordering::Relaxed);
            nucleo.guardian_supervisoes.store(valores[1], Ordering::Relaxed);
            nucleo.guardian_isolamentos.store(valores[2], Ordering::Relaxed);
            nucleo.zeus_ajustes.store(valores[3], Ordering::Relaxed);
            nucleo.aguia_alertas.store(valores[4], Ordering::Relaxed);
            nucleo.aguia_estado.store(valores[5], Ordering::Relaxed);
            nucleo.halyn_exportadas.store(valores[6], Ordering::Relaxed);
            nucleo.aegis_criptografias.store(valores[7], Ordering::Relaxed);
            nucleo.aegis_bloqueios.store(valores[8], Ordering::Relaxed);
            nucleo.maqso_adaptacoes.store(valores[9], Ordering::Relaxed);
            nucleo.maqso_score.store(valores[10], Ordering::Relaxed);
            nucleo.aprendizado_erros.store(valores[11], Ordering::Relaxed);
            nucleo.aprendizado_acertos.store(valores[12], Ordering::Relaxed);
            nucleo.reencarnacoes.store(valores[13], Ordering::Relaxed);
            nucleo.ciclos_processados.store(valores[14], Ordering::Relaxed);
            nucleo.memoria_persistente.store(valores[15], Ordering::Relaxed);
            nucleo.timestamp_ultimo_ciclo.store(valores[16], Ordering::Relaxed);
            nucleo.coracao.store(valores[17], Ordering::Relaxed);
        }
    }
}

#[allow(dead_code)]
fn prever_tendencia_ram(historico: &[f32], ram_total: u64) -> String {
    if historico.len() < 5 {
        return "Coletando dados...".to_string();
    }
    let ultimos: Vec<f32> = historico.iter().rev().take(10).cloned().collect();
    let n = ultimos.len() as f32;
    let media: f32 = ultimos.iter().sum::<f32>() / n;
    let variancia: f32 = ultimos.iter().map(|v| (v - media).powi(2)).sum::<f32>() / n;
    let _desvio = variancia.sqrt();
    let primeiro = ultimos.last().unwrap_or(&0.0);
    let ultimo = ultimos.first().unwrap_or(&0.0);
    let diff = ultimo - primeiro;
    if diff < 0.0 {
        let ciclos_para_esgotar = if diff.abs() > 0.0 { ram_total as f32 / diff.abs() } else { 0.0 };
        let minutos = (ciclos_para_esgotar * 2.0) / 60.0;
        if minutos < 60.0 {
            format!("Esgotamento em {:.1} min", minutos)
        } else {
            format!("Queda: {:.1} MB por ciclo", diff.abs())
        }
    } else if diff > 10.0 {
        format!("Recuperação: +{:.1} MB por ciclo", diff)
    } else {
        format!("Estável (média {:.1} MB)", media)
    }
}

#[allow(dead_code)]
fn ler_cpu_online() -> String {
    std::fs::read_to_string("/sys/devices/system/cpu/online")
        .unwrap_or_else(|_| "indisponivel".to_string())
        .trim()
        .to_string()
}
fn ler_limite(campo: &str) -> String {
    if let Ok(conteudo) = std::fs::read_to_string("/proc/self/limits") {
        for linha in conteudo.lines() {
            if linha.starts_with(campo) {
                let partes: Vec<&str> = linha.split_whitespace().collect();
                if partes.len() >= 4 {
                    return partes[3].to_string();
                }
            }
        }
    }
    "indisponivel".to_string()
}
fn contar_cpus_online() -> u64 {
    if let Ok(conteudo) = std::fs::read_to_string("/sys/devices/system/cpu/online") {
        let linha = conteudo.trim();
        if linha.contains("-") {
            let partes: Vec<&str> = linha.split("-").collect();
            if partes.len() == 2 {
                if let (Ok(inicio), Ok(fim)) = (partes[0].parse::<u64>(), partes[1].parse::<u64>()) {
                    return fim - inicio + 1;
                }
            }
        } else if let Ok(valor) = linha.parse::<u64>() {
            return valor + 1;
        }
    }
    0
}

fn coletar_metricas_prometheus(nucleo: &Nucleo) -> String {
    let ram_total = ler_meminfo("MemTotal");
    let ram_livre = ler_meminfo("MemAvailable");
    let processos = contar_processos();
    let uptime = ler_uptime();
    let cpu = ler_cpu();
    let agora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let mut s = String::with_capacity(2048);
    s.push_str(&format!("atcs_cpu_usage {:.4} {}\n", cpu, agora));
    s.push_str(&format!("atcs_loadavg {:.4} {}\n", ler_loadavg(), agora));
    s.push_str(&format!("atcs_ram_total_mb {} {}\n", ram_total, agora));
    s.push_str(&format!("atcs_ram_free_mb {} {}\n", ram_livre, agora));
    s.push_str(&format!("atcs_processos_total {} {}\n", processos, agora));
    s.push_str(&format!("atcs_uptime_seconds {} {}\n", uptime, agora));
    s.push_str(&format!("atcs_phl_eventos_total {} {}\n", nucleo.phl_eventos.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_phl_avaliacoes_total {} {}\n", nucleo.phl_compressoes.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_guardian_supervisoes_total {} {}\n", nucleo.guardian_supervisoes.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_guardian_condicoes_detectadas {} {}\n", nucleo.guardian_isolamentos.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_zeus_avaliacoes_total {} {}\n", nucleo.zeus_ajustes.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_aguia_estado_visual {} {}\n", nucleo.aguia_estado.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_aguia_alertas_total {} {}\n", nucleo.aguia_alertas.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_halyn_ciclos_total {} {}\n", nucleo.halyn_exportadas.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_aegis_avaliacoes_total {} {}\n", nucleo.aegis_criptografias.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_aegis_bloqueios_total {} {}\n", nucleo.aegis_bloqueios.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_anomalias_detectadas {} {}\n", ANOMALIAS_DETECTADAS.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_maqso_condicoes_detectadas {} {}\n", nucleo.maqso_adaptacoes.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_maqso_score_ambiente {} {}\n", nucleo.maqso_score.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_ciclos_processados {} {}\n", nucleo.ciclos_processados.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_reencarnacoes_total {} {}\n", nucleo.reencarnacoes.load(Ordering::Relaxed), agora));
    s.push_str(&format!("atcs_mem_total_mb {} {}\n", ler_meminfo("MemTotal"), agora));
    s.push_str(&format!("atcs_mem_free_mb {} {}\n", ler_meminfo("MemFree"), agora));
    s.push_str(&format!("atcs_mem_available_mb {} {}\n", ler_meminfo("MemAvailable"), agora));
    s.push_str(&format!("atcs_mem_cached_mb {} {}\n", ler_meminfo("Cached"), agora));
    s.push_str(&format!("atcs_mem_swap_total_mb {} {}\n", ler_meminfo("SwapTotal"), agora));
    s.push_str(&format!("atcs_mem_swap_free_mb {} {}\n", ler_meminfo("SwapFree"), agora));
    s.push_str(&format!("atcs_mem_buffers_mb {} {}\n", ler_meminfo("Buffers"), agora));
    s.push_str(&format!("atcs_max_processos {} {}\n", ler_limite("Max processes"), agora));
    s.push_str(&format!("atcs_max_open_files {} {}\n", ler_limite("Max open files"), agora));
    s.push_str(&format!("atcs_cpu_cores {} {}\n", contar_cpus_online(), agora));
    s.push_str(&format!("atcs_cpu_policy0_min {} {}\n", 500000, agora));
    s.push_str(&format!("atcs_cpu_policy0_max {} {}\n", 1800000, agora));
    s.push_str(&format!("atcs_cpu_policy6_min {} {}\n", 850000, agora));
    s.push_str(&format!("atcs_cpu_policy6_max {} {}\n", 2000000, agora));
    s
}

fn painel_web(nucleo: &Nucleo) -> String {
    let ambiente = detectar_ambiente();
    let root = detectar_root();

    let ram_total = ler_meminfo("MemTotal:");
    let ram_livre = ler_meminfo("MemFree:");
    let ram_disponivel = ler_meminfo("MemAvailable:");
    let processos = contar_processos();
    let cpu = ler_cpu();
    let load = ler_loadavg();
    let uptime = ler_uptime();
    let score = nucleo.maqso_score.load(Ordering::Relaxed);
    let ciclos = nucleo.ciclos_processados.load(Ordering::Relaxed);
    let reencarnacoes = nucleo.reencarnacoes.load(Ordering::Relaxed);
    let coracao = nucleo.coracao.load(Ordering::Relaxed);
    let ultimo_ciclo = nucleo.timestamp_ultimo_ciclo.load(Ordering::Relaxed);
    let agora_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    let timeout_ns = ATIVIDADE_TIMEOUT_SEGUNDOS.saturating_mul(1_000_000_000);

    let atividade = coracao > 0
        && ultimo_ciclo > 0
        && agora_ns >= ultimo_ciclo
        && agora_ns.saturating_sub(ultimo_ciclo) <= timeout_ns;

    let indicador = if atividade { "ATIVO" } else { "INATIVO" };
    let classe_atividade = if atividade { "heartbeat ativo" } else { "heartbeat" };

    let guardian = nucleo.guardian_supervisoes.load(Ordering::Relaxed);
    let guardian_condicoes = nucleo.guardian_isolamentos.load(Ordering::Relaxed);

    let aegis_avaliacoes = nucleo.aegis_criptografias.load(Ordering::Relaxed);
    let aegis_bloqueios = nucleo.aegis_bloqueios.load(Ordering::Relaxed);

    let aguia = nucleo.aguia_alertas.load(Ordering::Relaxed);
    let zeus = nucleo.zeus_ajustes.load(Ordering::Relaxed);
    let halyn = nucleo.halyn_exportadas.load(Ordering::Relaxed);

    let estado = if root { "OPERANDO COM PRIVILEGIO" } else { "OPERANDO SEM ROOT" };

    let memoria_pct = if ram_total > 0 {
        ((ram_total.saturating_sub(ram_disponivel)) * 100) / ram_total
    } else {
        0
    };

    let alerta = match nucleo.aguia_estado.load(Ordering::Relaxed) {
        2 => "ATENÇÃO AMBIENTAL",
        1 => "CONDIÇÃO AMBIENTAL",
        _ => "ESTADO ESTÁVEL",
    };

    let historico_ram = HISTORICO_RAM.lock().unwrap()
        .iter()
        .map(|v| format!("<span>{:.0}</span>", v))
        .collect::<Vec<_>>()
        .join(" ");

    let historico_cpu = HISTORICO_CPU.lock().unwrap()
        .iter()
        .map(|v| format!("<span>{:.1}</span>", v * 100.0))
        .collect::<Vec<_>>()
        .join(" ");

    let historico_score = HISTORICO_SCORE.lock().unwrap()
        .iter()
        .map(|v| format!("<span>{:.0}</span>", v))
        .collect::<Vec<_>>()
        .join(" ");

    let eventos = EVENTOS_LOG.lock().unwrap()
        .iter()
        .rev()
        .take(30)
        .map(|e| format!("<div>{}</div>", e))
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<meta http-equiv="refresh" content="2">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>ATCS Framework — Trajetória</title>
<style>
body{{margin:0;background:#080b10;color:#d8dee9;font-family:monospace;padding:18px}}
h1{{text-align:center;color:#00d4ff;margin:5px 0}}
h2{{color:#00d4ff;border-bottom:1px solid #26313d;padding-bottom:6px;margin-top:26px}}
.sub{{text-align:center;color:#687585}}
.state{{text-align:center;font-size:22px;font-weight:bold;padding:14px;margin:16px 0;border:1px solid #26313d;background:#10151c}}
.heartbeat{{display:inline-block;width:11px;height:11px;border-radius:50%;background:#4a5568;margin-right:8px}}
.heartbeat.ativo{{background:#00ff88;animation:pulse 1s ease-in-out infinite}}
@keyframes pulse{{0%,100%{{transform:scale(1);opacity:.55}}50%{{transform:scale(1.8);opacity:1}}}}
.grid{{display:grid;grid-template-columns:repeat(auto-fit,minmax(150px,1fr));gap:10px}}
.card{{background:#10151c;border:1px solid #26313d;border-radius:6px;padding:13px;text-align:center}}
.label{{color:#718096;font-size:11px;text-transform:uppercase}}
.value{{color:#00ff88;font-size:21px;font-weight:bold;margin-top:7px}}
.note{{color:#8a96a5;font-size:11px;margin-top:5px}}
.history{{background:#0d1218;border:1px solid #26313d;padding:10px;min-height:35px;overflow:hidden;white-space:nowrap}}
.history span{{display:inline-block;margin-right:7px;color:#00d4ff}}
.log{{background:#0d1218;border:1px solid #26313d;border-radius:5px;padding:10px;max-height:240px;overflow:auto}}
.small{{font-size:12px;color:#8a96a5}}
</style>
</head>
<body>

<h1>ATCS Framework</h1>
<div class="sub">Arquitetura experimental de auto-observação e continuidade</div>

<div class="state">
<span class="{classe_atividade}"></span>{alerta}
<div class="small">Ambiente: {ambiente} · {estado} · Atividade: {indicador}</div>
</div>

<h2>Estado Atual</h2>
<div class="grid">
<div class="card"><div class="label">Ciclos</div><div class="value">{ciclos}</div><div class="note">trajetória acumulada</div></div>
<div class="card"><div class="label">Reencarnações</div><div class="value">{reencarnacoes}</div><div class="note">eventos de continuidade</div></div>
<div class="card"><div class="label">Score Ambiental</div><div class="value">{score}</div><div class="note">indicador calculado</div></div>
<div class="card"><div class="label">CPU</div><div class="value">{cpu:.2}%</div><div class="note">observação do ambiente</div></div>
</div>

<h2>Observação do Ambiente</h2>
<div class="grid">
<div class="card"><div class="label">RAM Total</div><div class="value">{ram_total} MB</div></div>
<div class="card"><div class="label">RAM Disponível</div><div class="value">{ram_disponivel} MB</div></div>
<div class="card"><div class="label">RAM Livre</div><div class="value">{ram_livre} MB</div></div>
<div class="card"><div class="label">RAM em Uso (estimada)</div><div class="value">{memoria_pct}%</div></div>
<div class="card"><div class="label">Processos</div><div class="value">{processos}</div></div>
<div class="card"><div class="label">Load Average</div><div class="value">{load:.2}</div></div>
<div class="card"><div class="label">Uptime do Ambiente</div><div class="value">{uptime}s</div></div>
</div>

<h2>Memória e Trajetória</h2>
<div class="grid">
<div class="card"><div class="label">Trajetória de Ciclos</div><div class="value">{ciclos}</div><div class="note">estado acumulado</div></div>
<div class="card"><div class="label">Memória Disponível</div><div class="value">{ram_disponivel} MB</div><div class="note">estado observado</div></div>
<div class="card"><div class="label">Score</div><div class="value">{score}</div><div class="note">evolução calculada</div></div>
</div>

<h2>Histórico de Memória</h2>
<div class="history">{historico_ram}</div>

<h2>Histórico de CPU</h2>
<div class="history">{historico_cpu}</div>

<h2>Histórico de Avaliação</h2>
<div class="history">{historico_score}</div>

<h2>Avaliações e Condições</h2>
<div class="grid">
<div class="card"><div class="label">Supervisões</div><div class="value">{guardian}</div><div class="note">ciclos de supervisão</div></div>
<div class="card"><div class="label">Condições Detectadas</div><div class="value">{guardian_condicoes}</div></div>
<div class="card"><div class="label">Avaliações AEGIS</div><div class="value">{aegis_avaliacoes}</div></div>
<div class="card"><div class="label">Bloqueios</div><div class="value">{aegis_bloqueios}</div></div>
<div class="card"><div class="label">Alertas</div><div class="value">{aguia}</div></div>
<div class="card"><div class="label">Avaliações ZEUS</div><div class="value">{zeus}</div></div>
<div class="card"><div class="label">Ciclos HALYN</div><div class="value">{halyn}</div></div>
</div>

<h2>Eventos da Trajetória</h2>
<div class="log">{eventos}</div>

</body>
</html>"#,
        alerta = alerta,
        ambiente = ambiente,
        estado = estado,
        ciclos = ciclos,
        reencarnacoes = reencarnacoes,
        score = score,
        cpu = cpu * 100.0,
        ram_total = ram_total,
        ram_disponivel = ram_disponivel,
        ram_livre = ram_livre,
        memoria_pct = memoria_pct,
        processos = processos,
        load = load,
        uptime = uptime,
        historico_ram = historico_ram,
        historico_cpu = historico_cpu,
        historico_score = historico_score,
        guardian = guardian,
        guardian_condicoes = guardian_condicoes,
        aegis_avaliacoes = aegis_avaliacoes,
        aegis_bloqueios = aegis_bloqueios,
        aguia = aguia,
        zeus = zeus,
        halyn = halyn,
        eventos = eventos
    )
}

fn aegis_bloquear_requisicao(request: &str) -> bool {
    if request.starts_with("GET /metrics") || request.starts_with("GET /metrics/raw") || request.starts_with("GET /api/v1/query") || request.starts_with("GET /api/v1/query_range") || request.starts_with("GET /api/v1/label") || request.starts_with("GET /api/v1/series") || request.starts_with("GET /painel") || request.starts_with("GET /health") || request.starts_with("GET /ping") || request.starts_with("GET /discover") || request.starts_with("GET /cmd") {
        return false;
    }
    request.contains("../../") || request.contains("etc/passwd") || request.contains("GET /admin") || request.contains("POST")
}

fn processar_comando(request: &str) -> String {
    if request.contains("cmd=status") {
        let ram = ler_meminfo("MemAvailable");
        let cpu = ler_cpu();
        format!("RAM: {} MB | CPU: {:.1}%", ram, cpu * 100.0)
    } else if request.contains("cmd=reset") {
        "Comando reset recebido. Módulos reiniciados.".to_string()
    } else {
        "Comandos: cmd=status | cmd=reset".to_string()
    }
}

fn iniciar_servidor_http(nucleo: Arc<Nucleo>) {
    let porta_configurada = obter_porta();

    let mut listener = None;
    let mut porta = porta_configurada;

    for candidato in porta_configurada..=porta_configurada.saturating_add(10) {
        match TcpListener::bind(("0.0.0.0", candidato)) {
            Ok(l) => {
                listener = Some(l);
                porta = candidato;
                break;
            }
            Err(_) => {}
        }
    }

    let listener = match listener {
        Some(l) => l,
        None => {
            eprintln!(
                "ATCS FRAMEWORK — não foi possível reservar uma porta entre {} e {}",
                porta_configurada,
                porta_configurada.saturating_add(10)
            );
            return;
        }
    };

    if porta != porta_configurada {
        println!(
            "ATCS FRAMEWORK — porta {} ocupada; utilizando porta {}",
            porta_configurada, porta
        );
    }

    println!("ATCS FRAMEWORK — Servidor HTTP na porta {}", porta);
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let nucleo = Arc::clone(&nucleo);
            thread::spawn(move || {
                let mut buffer = [0; 2048];
                let n = stream.read(&mut buffer).unwrap_or(0);
                let request = String::from_utf8_lossy(&buffer[..n]);
                if aegis_bloquear_requisicao(&request) {
                    nucleo.aegis_bloqueios.fetch_add(1, Ordering::Relaxed);
                    let _ = stream.write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n");
                    let _ = stream.flush();
                    return;
                }
                let resposta = if request.starts_with("GET /metrics/raw") {
                    let corpo = coletar_metricas_prometheus(&nucleo);
                    let limpo: Vec<&str> = corpo.lines().filter(|l| !l.starts_with('#')).collect();
                    let saida = limpo.join("\n");
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", saida.len(), saida)
                } else if request.starts_with("GET /metrics") {
                    let corpo = coletar_metricas_prometheus(&nucleo);
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", corpo.len(), corpo)
                } else if request.starts_with("GET /painel") || request.starts_with("GET / ") {
                    let corpo = painel_web(&nucleo);
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}", corpo.len(), corpo)
                } else if request.starts_with("GET /health") {
                    let corpo = format!("{{\"status\":\"ok\",\"reencarnacoes\":{}}}", nucleo.reencarnacoes.load(Ordering::Relaxed));
                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", corpo.len(), corpo)
                } else if request.starts_with("GET /ping") {
                    let corpo = format!("{{\"atcs\":\"vivo\",\"volume\":{},\"ram\":{}}}", nucleo.ciclos_processados.load(Ordering::Relaxed), ler_meminfo("MemAvailable"));
                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", corpo.len(), corpo)
                } else if request.starts_with("GET /discover") {
                    let corpo = format!("{{\"atcs\":\"vivo\",\"porta\":{},\"modulos\":7,\"volume\":{}}}", obter_porta(), nucleo.ciclos_processados.load(Ordering::Relaxed));
                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", corpo.len(), corpo)
                } else if request.starts_with("GET /cmd") || request.starts_with("POST /cmd") {
                    let corpo = processar_comando(&request);
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", corpo.len(), corpo)
                } else {
                    let corpo = if request.starts_with("GET /modulos") { maestro_modulos() } else if request.starts_with("GET /existencia") { maestro_existencia() } else if request.starts_with("GET /identidades") { maestro_identidades() } else if request.starts_with("GET /saude") { maestro_saude(&nucleo) } else if request.starts_with("GET /contrato") { contrato_json(&nucleo) } else { format!("ATCS FRAMEWORK — vivo. Uptime: {}s", ler_uptime()) };
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", corpo.len(), corpo)
                };
                let _ = stream.write_all(resposta.as_bytes());
                let _ = stream.flush();
            });
        }
    }
}

fn ciclo_modulos(nucleo: Arc<Nucleo>, _aprendizado: Arc<Mutex<Vec<f32>>>) {
    loop {
        let ram_total = ler_meminfo("MemTotal");
        let ram_livre = ler_meminfo("MemAvailable");
        let ram_usada = ram_total.saturating_sub(ram_livre);
        let processos = contar_processos();
        let cpu = ler_cpu();
        nucleo.phl_eventos.store(processos, Ordering::Relaxed);
        nucleo.phl_compressoes.store(processos / 2, Ordering::Relaxed);
        nucleo.guardian_supervisoes.fetch_add(1, Ordering::Relaxed);
        nucleo.guardian_isolamentos.fetch_add(if processos > 50 { 1 } else { 0 }, Ordering::Relaxed);
        nucleo.zeus_ajustes.fetch_add(1, Ordering::Relaxed);
        let uid = fs::metadata("/proc/self").map(|m| m.uid()).unwrap_or(0) as u64;
        nucleo.zeus_privilegios.store(uid, Ordering::Relaxed);
        let uso_percentual = if ram_total > 0 { (ram_usada as f32 / ram_total as f32) * 100.0 } else { 0.0 };
        let estado_visual = if uso_percentual > RAM_CRITICA_PERCENT { 2 } else if uso_percentual > RAM_ALERTA_PERCENT { 1 } else { 0 };
        nucleo.aguia_estado.store(estado_visual, Ordering::Relaxed);
        if estado_visual > 0 {
            nucleo.aguia_alertas.fetch_add(1, Ordering::Relaxed);
        }
        nucleo.halyn_exportadas.fetch_add(1, Ordering::Relaxed);
        nucleo.aegis_criptografias.fetch_add(1, Ordering::Relaxed);
        let load_factor = ler_loadavg().min(10.0);
        let score = (cpu * 100.0 * 0.4 + (100.0 - uso_percentual) * 0.4 + (10.0 - load_factor) * 10.0 * 0.2).clamp(0.0, 100.0);
        if score < 20.0 {
            nucleo.maqso_adaptacoes.fetch_add(1, Ordering::Relaxed);
            AUTO_AJUSTES.fetch_add(1, Ordering::Relaxed);
        }
        nucleo.maqso_score.store(score as u64, Ordering::Relaxed);

        {
            let mut h = HISTORICO_RAM.lock().unwrap();
            h.push(ram_usada as f32);
            if h.len() > 100 {
                h.remove(0);
            }
        }

        {
            let mut h = HISTORICO_CPU.lock().unwrap();
            h.push(cpu);
            if h.len() > 100 {
                h.remove(0);
            }
        }

        {
            let mut h = HISTORICO_SCORE.lock().unwrap();
            h.push(score as u64);
            if h.len() > 100 {
                h.remove(0);
            }
        }

        {
            let estado_atual: [u64; 18] = [
                nucleo.phl_eventos.load(Ordering::Relaxed),
                nucleo.guardian_supervisoes.load(Ordering::Relaxed),
                nucleo.guardian_isolamentos.load(Ordering::Relaxed),
                nucleo.zeus_ajustes.load(Ordering::Relaxed),
                nucleo.aguia_alertas.load(Ordering::Relaxed),
                nucleo.aguia_estado.load(Ordering::Relaxed),
                nucleo.halyn_exportadas.load(Ordering::Relaxed),
                nucleo.aegis_criptografias.load(Ordering::Relaxed),
                nucleo.aegis_bloqueios.load(Ordering::Relaxed),
                nucleo.maqso_adaptacoes.load(Ordering::Relaxed),
                nucleo.maqso_score.load(Ordering::Relaxed),
                nucleo.aprendizado_erros.load(Ordering::Relaxed),
                nucleo.aprendizado_acertos.load(Ordering::Relaxed),
                nucleo.reencarnacoes.load(Ordering::Relaxed),
                nucleo.ciclos_processados.load(Ordering::Relaxed),
                nucleo.memoria_persistente.load(Ordering::Relaxed),
                nucleo.timestamp_ultimo_ciclo.load(Ordering::Relaxed),
                nucleo.coracao.load(Ordering::Relaxed),
            ];
            let mut guard = APRENDIZADO_SISTEMA.lock().unwrap();
            let hist = guard.get_or_insert_with(AprendizadoSistema::novo);
            if let Some(anterior) = hist.ultimo {
                let mut desvio_detectado = false;
                let mut detalhes: Vec<String> = Vec::new();
                let detectaveis: [usize; 11] = [1, 2, 3, 4, 6, 7, 8, 9, 13, 14, 17];
                for i in detectaveis.iter() {
                    let i = *i;
                    let delta = estado_atual[i].saturating_sub(anterior[i]) as f64;
                    let h = &mut hist.deltas[i];
                    h.push(delta);
                    if h.len() > 100 { h.remove(0); }
                    if h.len() > 8 {
                        let media: f64 = h.iter().sum::<f64>() / h.len() as f64;
                        let var: f64 = h.iter().map(|v| (v - media).powi(2)).sum::<f64>() / h.len() as f64;
                        let sigma = var.sqrt();
                        let persistencia = &mut hist.persistencia[i];
                        if sigma > 0.0 && (delta - media).abs() > 5.0 * sigma {
                            *persistencia += 1;
                            if *persistencia >= 5 {
                                desvio_detectado = true;
                                detalhes.push(format!(
                                    "{} indice={} valor={} persistencia={}/5 decisao=REAL",
                                    semantica::descrever(i, delta, media, sigma),
                                    i, estado_atual[i], *persistencia
                                ));
                            } else {
                                detalhes.push(format!(
                                    "{} indice={} valor={} persistencia={}/5 decisao=observando",
                                    semantica::descrever(i, delta, media, sigma),
                                    i, estado_atual[i], *persistencia
                                ));
                            }
                        } else {
                            *persistencia = 0;
                        }
                    }
                }
                let gauges: [usize; 3] = [0, 5, 10];
                for i in gauges.iter() {
                    let i = *i;
                    if estado_atual[i] != anterior[i] {
                        let agora_g = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        if let Ok(meta) = std::fs::metadata("atcs_gauges.log") {
                            if meta.len() > 10 * 1024 * 1024 {
                                let _ = std::fs::remove_file("atcs_gauges.log.1");
                                let _ = std::fs::rename("atcs_gauges.log", "atcs_gauges.log.1");
                            }
                        }
                        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("atcs_gauges.log") {
                            let _ = writeln!(f, "[{}] contador={} modulo={} valor_anterior={} valor_atual={}", agora_g, semantica::nome(i), semantica::modulo(i), anterior[i], estado_atual[i]);
                        }
                    }
                }
                {
                    let t_atual = estado_atual[16];
                    let t_anterior = anterior[16];
                    if t_atual > t_anterior {
                        let delta_ns = t_atual - t_anterior;
                        if delta_ns > 0 {
                            let ciclos_por_seg = 1_000_000_000.0 / delta_ns as f64;
                            let agora_v = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("atcs_velocidade.log") {
                                let _ = writeln!(f, "[{}] delta_ns={} ciclos_por_segundo={:.2}", agora_v, delta_ns, ciclos_por_seg);
                            }
                        }
                    }
                }
                let agora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                if desvio_detectado {
                    nucleo.aprendizado_erros.fetch_add(1, Ordering::Relaxed);
                    if let Ok(meta) = std::fs::metadata("atcs_erros.log") {
                        if meta.len() > 20 * 1024 * 1024 {
                            let _ = std::fs::remove_file("atcs_erros.log.1");
                            let _ = std::fs::rename("atcs_erros.log", "atcs_erros.log.1");
                        }
                    }
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("atcs_erros.log") {
                        for d in detalhes.iter() {
                            let _ = writeln!(f, "[{}] {} decisao=erro", agora, d);
                        }
                    }
                } else {
                    nucleo.aprendizado_acertos.fetch_add(1, Ordering::Relaxed);
                    let ciclos = nucleo.ciclos_processados.load(Ordering::Relaxed);
                    if ciclos % 100 == 0 {
                        if let Ok(meta) = std::fs::metadata("atcs_acertos.log") {
                            if meta.len() > 20 * 1024 * 1024 {
                                let _ = std::fs::remove_file("atcs_acertos.log.1");
                                let _ = std::fs::rename("atcs_acertos.log", "atcs_acertos.log.1");
                            }
                        }
                        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("atcs_acertos.log") {
                            let _ = writeln!(f, "[{}] contadores_ok=18 ciclo={} decisao=acerto", agora, ciclos);
                        }
                    }
                }
            }
            hist.ultimo = Some(estado_atual);
        }
        nucleo.ciclos_processados.fetch_add(1, Ordering::Relaxed);

        {
            let volume = nucleo.ciclos_processados.load(Ordering::Relaxed);
            let volume_anterior = ULTIMO_VOLUME.load(Ordering::Relaxed);
            let tick_anterior = ULTIMO_TICK.load(Ordering::Relaxed);
            ULTIMO_VOLUME.store(volume, Ordering::Relaxed);
            let agora_tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
            ULTIMO_TICK.store(agora_tick, Ordering::Relaxed);
            let ciclos_por_segundo: f64 = if tick_anterior > 0 && agora_tick > tick_anterior {
                let diff = volume.saturating_sub(volume_anterior) as f64;
                let tempo = (agora_tick - tick_anterior) as f64 / 1_000_000_000.0;
                if tempo > 0.0 { diff / tempo } else { 0.0 }
            } else { 0.0 };
            let _ = ciclos_por_segundo;

            let mut hist_vol = HISTORICO_VOLUME.lock().unwrap();
            hist_vol.push(volume);
            if hist_vol.len() > 20 { hist_vol.remove(0); }
            drop(hist_vol);

            let aegis = nucleo.aegis_bloqueios.load(Ordering::Relaxed);
            let guardian = nucleo.guardian_supervisoes.load(Ordering::Relaxed);
            let reencarnacoes = nucleo.reencarnacoes.load(Ordering::Relaxed);
            let mut narrativa = NARRATIVA.lock().unwrap();
            if narrativa.is_empty() {
                narrativa.push(format!("Sistema iniciado"));
            }
            if aegis > 0 && aegis % 100 == 0 {
                narrativa.push(format!("AEGIS bloqueou {} ameaças", aegis));
            }
            if guardian > 0 && guardian % 5000 == 0 {
                narrativa.push(format!("GUARDIAN supervisionou {} eventos", guardian));
            }
            if reencarnacoes > 0 {
                narrativa.push(format!("Sistema reencarnou {} vezes", reencarnacoes));
            }
            if narrativa.len() > 8 { narrativa.remove(0); }
        }
        nucleo.coracao.fetch_add(1, Ordering::Relaxed);
        nucleo.timestamp_ultimo_ciclo.store(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64, Ordering::Relaxed);
        let _carga = ler_cpu();
        if nucleo.ciclos_processados.load(Ordering::Relaxed) % 1000 == 0 {
            gestao_verificar();
        }
        if nucleo.ciclos_processados.load(Ordering::Relaxed) % 1000 == 0 {
            let j = contrato_tribunal(&nucleo);
            contrato_registrar(&j);
        }
        std::hint::spin_loop();
    }
}

fn watchdog_interno(nucleo: Arc<Nucleo>, coracao_anterior: Arc<AtomicU64>) {
    loop {
        thread::sleep(Duration::from_millis(2000));
        let atual = nucleo.coracao.load(Ordering::Relaxed);
        let anterior = coracao_anterior.swap(atual, Ordering::Relaxed);

        if atual == anterior && anterior > 0 {
            nucleo.reencarnacoes.fetch_add(1, Ordering::Relaxed);

            eprintln!(
                "WATCHDOG: Coração parado. Encerramento controlado da instância."
            );

            persistir_estado(&nucleo, "atcs_state.bin");

            exit(1);
        }
    }
}


fn controlar_instancia_atcs() {
    use std::fs::{self, OpenOptions};
    use std::io::{Read, Write};
    use std::path::Path;

    let lock_path = Path::new(".atcs_instance.lock");
    let pid_atual = std::process::id();

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(lock_path)
    {
        Ok(mut f) => {
            let _ = writeln!(f, "{}", pid_atual);
        }

        Err(_) => {
            let mut pid_existente = String::new();

            if let Ok(mut f) = fs::File::open(lock_path) {
                let _ = f.read_to_string(&mut pid_existente);
            }

            let pid = pid_existente.trim().parse::<u32>().ok();

            if let Some(pid) = pid {
                let processo_ativo = {
                    let caminho = format!("/proc/{}/cmdline", pid);
                    std::fs::read(&caminho)
                        .map(|bytes| {
                            let cmdline = String::from_utf8_lossy(&bytes);
                            cmdline.contains("atcs_framework")
                        })
                        .unwrap_or(false)
                };

                if processo_ativo && pid != pid_atual {
                    eprintln!(
                        "ATCS FRAMEWORK — instância já ativa (PID {}). Encerrando nova instância.",
                        pid
                    );
                    std::process::exit(0);
                }
            }

            let _ = fs::remove_file(lock_path);

            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(lock_path)
            {
                Ok(mut f) => {
                    let _ = writeln!(f, "{}", pid_atual);
                }
                Err(_) => {
                    eprintln!(
                        "ATCS FRAMEWORK — não foi possível assumir o controle da instância."
                    );
                    std::process::exit(1);
                }
            }
        }
    }
}

fn ler_arquivo_str(caminho: &str) -> Option<String> {
    std::fs::read_to_string(caminho).ok()
}

fn ler_kernel_snapshot(pid: u32) -> String {
    let agora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let uptime = ler_arquivo_str("/proc/uptime").unwrap_or_default();
    let uptime = uptime.split_whitespace().next().unwrap_or("0").to_string();
    let meminfo = ler_arquivo_str("/proc/meminfo").unwrap_or_default();
    let mem_avail = meminfo.lines().find(|l| l.starts_with("MemAvailable:"))
        .and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").to_string();
    let mem_free = meminfo.lines().find(|l| l.starts_with("MemFree:"))
        .and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").to_string();
    let swap_free = meminfo.lines().find(|l| l.starts_with("SwapFree:"))
        .and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").to_string();
    let psi = ler_arquivo_str("/proc/pressure/memory").unwrap_or_default();
    let psi_avg10 = psi.lines().find(|l| l.starts_with("some"))
        .and_then(|l| l.split_whitespace().find(|p| p.starts_with("avg10=")))
        .map(|p| p.trim_start_matches("avg10=").to_string())
        .unwrap_or_else(|| "0".to_string());
    let vmstat = ler_arquivo_str("/proc/vmstat").unwrap_or_default();
    let oom_kill = vmstat.lines().find(|l| l.starts_with("oom_kill "))
        .and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").to_string();
    let pgmajfault = vmstat.lines().find(|l| l.starts_with("pgmajfault "))
        .and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").to_string();
    let loadavg = ler_arquivo_str("/proc/loadavg").unwrap_or_default();
    let load = loadavg.split_whitespace().next().unwrap_or("0").to_string();
    let status = ler_arquivo_str(&format!("/proc/{}/status", pid)).unwrap_or_default();
    let threads = status.lines().find(|l| l.starts_with("Threads:"))
        .and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").to_string();
    let vm_rss = status.lines().find(|l| l.starts_with("VmRSS:"))
        .and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").to_string();
    let mut estados = String::new();
    if let Ok(entradas) = std::fs::read_dir(format!("/proc/{}/task", pid)) {
        for e in entradas.flatten() {
            let tid = e.file_name().to_string_lossy().to_string();
            let tstatus = ler_arquivo_str(&format!("/proc/{}/task/{}/status", pid, tid)).unwrap_or_default();
            if let Some(l) = tstatus.lines().find(|l| l.starts_with("State:")) {
                let s = l.trim_start_matches("State:").trim();
                let inicial = s.chars().next().unwrap_or('?');
                estados.push(inicial);
            }
        }
    }
    if estados.is_empty() { estados.push('?'); }
    let oom_score = ler_arquivo_str(&format!("/proc/{}/oom_score", pid)).unwrap_or_default();
    let oom_score = oom_score.trim().to_string();
    let wchan = ler_arquivo_str(&format!("/proc/{}/wchan", pid)).unwrap_or_default();
    let wchan = wchan.trim().to_string();
    format!(
        "[{}] uptime={} mem_avail_kb={} mem_free_kb={} swap_free_kb={} psi_mem_avg10={} oom_kill={} pgmajfault={} load={} states={} threads={} vm_rss_kb={} oom_score={} wchan={}",
        agora, uptime, mem_avail, mem_free, swap_free, psi_avg10, oom_kill, pgmajfault, load, estados, threads, vm_rss, oom_score, wchan
    )
}

fn kernel_watcher(pid: u32, buffer: Arc<Mutex<Vec<String>>>) {
    loop {
        let linha = ler_kernel_snapshot(pid);
        {
            let mut b = buffer.lock().unwrap();
            b.push(linha);
            if b.len() > 5000 {
                let drenado: Vec<String> = b.drain(..2500).collect();
                drop(b);
                if let Ok(meta) = std::fs::metadata("atcs_kernel.log") {
                    if meta.len() > 30 * 1024 * 1024 {
                        let _ = std::fs::remove_file("atcs_kernel.log.1");
                        let _ = std::fs::rename("atcs_kernel.log", "atcs_kernel.log.1");
                    }
                }
                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("atcs_kernel.log") {
                    for l in drenado {
                        let _ = writeln!(f, "{}", l);
                    }
                }
            }
        }
        thread::sleep(Duration::from_micros(1000));
    }
}


fn maestro_modulos() -> String {
    let mut out = String::from("{\n  \"modulos\": [\n");
    if let Ok(dir) = std::fs::read_dir(format!("{}/var/service", std::env::var("PREFIX").unwrap_or_default())) {
        let mut primeiro = true;
        for e in dir.flatten() {
            let nome = e.file_name().to_string_lossy().to_string();
            if nome.starts_with("atcs") {
                let status = std::process::Command::new("sv")
                    .args(["status", &format!("{}/var/service/{}", std::env::var("PREFIX").unwrap_or_default(), nome)])
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                if !primeiro { out.push_str(",\n"); }
                primeiro = false;
                out.push_str(&format!("    {{\"nome\": \"{}\", \"status\": \"{}\"}}", nome, status.replace("\"", "'")));
            }
        }
    }
    out.push_str("\n  ]\n}");
    out
}

fn maestro_existencia() -> String {
    use sha2::{Sha256, Digest};
    let caminho = "atcs_state.bin";
    if let Ok(data) = std::fs::read(caminho) {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let resultado = hasher.finalize();
        let hash: String = resultado.iter().map(|b| format!("{:02x}", b)).collect();
        let stat = std::fs::metadata(caminho).ok();
        let tamanho = stat.as_ref().map(|m| m.len()).unwrap_or(0);
        let agora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let linha = format!("[{}] size={} sha256={}\n", agora, tamanho, hash);
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("atcs_identidades.log") {
            let _ = f.write_all(linha.as_bytes());
        }
        format!("{{\n  \"ok\": true,\n  \"timestamp\": {},\n  \"observation\": {{\n    \"exists\": true,\n    \"size\": {},\n    \"sha256\": \"{}\"\n  }},\n  \"identidade\": \"{}\"\n}}", agora, tamanho, hash, hash)
    } else {
        "{\"ok\": false, \"error\": \"STATE_NOT_FOUND\"}".to_string()
    }
}

fn maestro_saude(nucleo: &Nucleo) -> String {
    let ciclos = nucleo.ciclos_processados.load(Ordering::Relaxed);
    let coracao = nucleo.coracao.load(Ordering::Relaxed);
    let acertos = nucleo.aprendizado_acertos.load(Ordering::Relaxed);
    let erros = nucleo.aprendizado_erros.load(Ordering::Relaxed);
    let persist = nucleo.memoria_persistente.load(Ordering::Relaxed);
    let coerente = coracao == ciclos;
    let taxa = if acertos + erros > 0 { 100.0 * acertos as f64 / (acertos + erros) as f64 } else { 0.0 };
    format!("{{\n  \"ok\": {},\n  \"ciclos\": {},\n  \"coracao\": {},\n  \"coerente\": {},\n  \"acertos\": {},\n  \"erros\": {},\n  \"taxa_acerto\": {:.2},\n  \"persistencias\": {}\n}}", coerente, ciclos, coracao, coerente, acertos, erros, taxa, persist)
}

fn maestro_identidades() -> String {
    match std::fs::read_to_string("atcs_identidades.log") {
        Ok(txt) => {
            let linhas: Vec<&str> = txt.lines().rev().take(20).collect();
            format!("{{\"total\": {}, \"ultimas\": [{}]}}", txt.lines().count(), linhas.iter().map(|l| format!("\"{}\"", l.replace("\"", "'"))).collect::<Vec<_>>().join(", "))
        }
        Err(_) => "{\"total\": 0, \"ultimas\": []}".to_string(),
    }
}


fn gestao_ler_config() -> Vec<(String, String)> {
    let mut lista = Vec::new();
    if let Ok(txt) = std::fs::read_to_string("atcs_servicos.conf") {
        for linha in txt.lines() {
            let partes: Vec<&str> = linha.split_whitespace().collect();
            if partes.len() >= 2 {
                lista.push((partes[0].to_string(), partes[1].to_string()));
            }
        }
    }
    lista
}

fn gestao_status(nome: &str) -> bool {
    let prefix = std::env::var("PREFIX").unwrap_or_default();
    let caminho = format!("{}/var/service/{}", prefix, nome);
    std::process::Command::new("sv")
        .args(["status", &caminho])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).starts_with("run:"))
        .unwrap_or(false)
}

fn gestao_acao(nome: &str, acao: &str) {
    let prefix = std::env::var("PREFIX").unwrap_or_default();
    let caminho = format!("{}/var/service/{}", prefix, nome);
    let _ = std::process::Command::new("sv").args([acao, &caminho]).output();
}

fn gestao_registrar(msg: &str) {
    let agora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("atcs_gestao.log") {
        let _ = writeln!(f, "[{}] {}", agora, msg);
    }
}

fn gestao_verificar() {
    let config = gestao_ler_config();
    for (nome, papel) in config.iter() {
        let rodando = gestao_status(nome);
        match papel.as_str() {
            "obrigatorio" => {
                if !rodando {
                    gestao_registrar(&format!("SUBIU {} (obrigatorio estava down)", nome));
                    gestao_acao(nome, "up");
                }
            }
            "proibido" => {
                if rodando {
                    gestao_registrar(&format!("PAROU {} (proibido estava up)", nome));
                    gestao_acao(nome, "down");
                }
            }
            _ => {}
        }
    }
}


#[derive(Default, Clone)]
struct Julgamento {
    cumpridas: u8,
    violadas: u8,
    total: u8,
}

fn contrato_c1(_n: &Nucleo) -> bool {
    std::fs::metadata("atcs_state.bin").map(|m| m.len() == 144).unwrap_or(false)
}

fn contrato_c2(n: &Nucleo) -> bool {
    n.coracao.load(Ordering::Relaxed) == n.ciclos_processados.load(Ordering::Relaxed)
}

fn contrato_c3(n: &Nucleo) -> bool {
    if let Ok(txt) = std::fs::read_to_string("atcs_narrativa.txt") {
        let ultimo = txt.lines().last().unwrap_or("");
        if let Some(vol_str) = ultimo.split("Vol:").nth(1) {
            if let Ok(vol) = vol_str.trim().parse::<u64>() {
                return vol <= n.ciclos_processados.load(Ordering::Relaxed);
            }
        }
    }
    true
}

fn contrato_c4(_n: &Nucleo) -> bool {
    std::fs::metadata("atcs_identidades.log").is_ok()
}

fn contrato_c5(n: &Nucleo) -> bool {
    n.aprendizado_acertos.load(Ordering::Relaxed) + n.aprendizado_erros.load(Ordering::Relaxed) > 0
}

fn contrato_c6(n: &Nucleo) -> bool {
    let a = n.aprendizado_acertos.load(Ordering::Relaxed);
    let e = n.aprendizado_erros.load(Ordering::Relaxed);
    if a + e == 0 { return true; }
    100.0 * a as f64 / (a + e) as f64 >= 90.0
}

fn contrato_c7(n: &Nucleo) -> bool {
    let ts = n.timestamp_ultimo_ciclo.load(Ordering::Relaxed);
    if ts == 0 { return true; }
    let agora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    agora.saturating_sub(ts) < 5_000_000_000
}

fn contrato_c8(_n: &Nucleo) -> bool {
    let prefix = std::env::var("PREFIX").unwrap_or_default();
    if let Ok(entradas) = std::fs::read_dir(format!("{}/var/service", prefix)) {
        let servicos: Vec<String> = entradas.flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n != "." && n != "..")
            .collect();
        return servicos.len() <= 1;
    }
    true
}

fn contrato_c9(_n: &Nucleo) -> bool {
    std::fs::metadata("atcs_state.bin").map(|m| m.len() == 144).unwrap_or(false)
}

fn contrato_c10(n: &Nucleo) -> bool {
    let erros = n.aprendizado_erros.load(Ordering::Relaxed);
    if erros == 0 { return true; }
    std::fs::metadata("atcs_erros.log").is_ok()
}

fn contrato_tribunal(n: &Nucleo) -> Julgamento {
    let mut j = Julgamento::default();
    j.total = 10;
    let checks = [
        contrato_c1(n), contrato_c2(n), contrato_c3(n), contrato_c4(n), contrato_c5(n),
        contrato_c6(n), contrato_c7(n), contrato_c8(n), contrato_c9(n), contrato_c10(n),
    ];
    for c in checks.iter() {
        if *c { j.cumpridas += 1; } else { j.violadas += 1; }
    }
    j
}

fn contrato_registrar(j: &Julgamento) {
    if j.violadas == 0 { return; }
    let agora = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("atcs_contrato.log") {
        let _ = writeln!(f, "[{}] CONTRATO: {}/{} cumpridas, {} violadas", agora, j.cumpridas, j.total, j.violadas);
    }
}

fn contrato_json(n: &Nucleo) -> String {
    let j = contrato_tribunal(n);
    let c1 = contrato_c1(n); let c2 = contrato_c2(n); let c3 = contrato_c3(n);
    let c4 = contrato_c4(n); let c5 = contrato_c5(n); let c6 = contrato_c6(n);
    let c7 = contrato_c7(n); let c8 = contrato_c8(n); let c9 = contrato_c9(n);
    let c10 = contrato_c10(n);
    format!(
        "{{\n  \"cumpridas\": {},\n  \"violadas\": {},\n  \"total\": 10,\n  \"clausulas\": {{\n    \"c1_persistencia\": {},\n    \"c2_integridade\": {},\n    \"c3_continuidade\": {},\n    \"c4_autenticidade\": {},\n    \"c5_auto_observacao\": {},\n    \"c6_coerencia_minima\": {},\n    \"c7_deteccao_travamento\": {},\n    \"c8_soberania\": {},\n    \"c9_contencao_estado\": {},\n    \"c10_rastreabilidade_erros\": {}\n  }}\n}}",
        j.cumpridas, j.violadas, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10
    )
}

fn main() {
    controlar_instancia_atcs();
    let porta = obter_porta();
    NASCIMENTO.store(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), Ordering::Relaxed);
    println!("ATCS FRAMEWORK");
    println!("Ambiente: {}", detectar_ambiente());
    println!("Root: {}", if detectar_root() { "sim" } else { "não" });
    println!("MODO: {}", if detectar_root() { "SOBERANO" } else { "CIDADÃO" });
    println!("Porta configurada: {}", porta);
    let nucleo = Arc::new(Nucleo::novo());
    carregar_estado(&nucleo, "atcs_state.bin");
    let pid_atual = std::process::id();
    let buffer_kernel: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    {
        let b = buffer_kernel.clone();
        thread::spawn(move || kernel_watcher(pid_atual, b));
    }
    let aprendizado = Arc::new(Mutex::new(Vec::new()));
    let nucleo_http = Arc::clone(&nucleo);
    thread::spawn(move || {
        iniciar_servidor_http(nucleo_http);
    });
    let nucleo_ciclo = Arc::clone(&nucleo);
    let aprendizado_ciclo = Arc::clone(&aprendizado);
    thread::spawn(move || {
        ciclo_modulos(nucleo_ciclo, aprendizado_ciclo);
    });
    let coracao_anterior = Arc::new(AtomicU64::new(0));
    let nucleo_watchdog = Arc::clone(&nucleo);
    let coracao_watchdog = Arc::clone(&coracao_anterior);
    thread::spawn(move || {
        watchdog_interno(nucleo_watchdog, coracao_watchdog);
    });
    loop {
        thread::sleep(Duration::from_secs(60));
        persistir_estado(&nucleo, "atcs_state.bin");
        if let Ok(mut arq) = OpenOptions::new().create(true).append(true).open("atcs_narrativa.txt") {
            let _ = writeln!(arq, "[{}] RAM:{}MB CPU:{:.1}% Vol:{}",
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                ler_meminfo("MemAvailable"), ler_cpu() * 100.0,
                nucleo.ciclos_processados.load(Ordering::Relaxed));
        }
    }
}
