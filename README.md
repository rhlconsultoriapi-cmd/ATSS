# ATCS Framework/Linux

Sistema de monitoramento e persistência de estado para ambientes Linux.

## Requisitos
- Linux (qualquer distro)
- Rust (para compilar) ou Docker (para rodar em container)

## Compilar
rustc -C target-cpu=native -C opt-level=3 -C lto=fat -C panic=abort -C codegen-units=1 -C strip=symbols src/main.rs -o atcs_framework

## Rodar
./atcs_framework

## Rodar via Docker
docker build -t atcs .
docker run -p 8082:8082 atcs

## Endpoints
/metrics - Métricas Prometheus (formato texto)
/health - Status do sistema
/painel - Painel HTML com métricas em tempo real

## Ambiente
- Linux (qualquer distro)
- Docker (container)
- Detecção real de root (uid)
- Detecção real de ambiente (Linux/Docker)
