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


ATCS Framework
Engenharia de Sistemas · Inteligência Computacional · Autonomia Arquitetural
Autor e idealizador: River Feitosa
Iniciativa: Intelligence Global
Projeto: ATCS Framework
Status: Em desenvolvimento independente
Sobre o projeto
O ATCS Framework é um trabalho independente de engenharia de sistemas dedicado à investigação, construção, integração e evolução de arquiteturas computacionais.
Sua linha de desenvolvimento reúne áreas como observabilidade, monitoramento de estados, integridade operacional, inteligência distribuída, coordenação entre subsistemas, análise de eventos e mecanismos de resposta.
O projeto parte de uma orientação central: compreender os mecanismos internos dos sistemas, investigar seu comportamento e desenvolver soluções arquiteturais próprias.
Compreender profundamente para construir com independência.
Eixos de engenharia
O trabalho organiza competências por meio de quatro referências bioinspiradas. Elas representam domínios funcionais de engenharia, e não uma reprodução literal dos organismos.
1. Mosca --- percepção e observabilidade
Referência para mecanismos de percepção contínua do ambiente computacional.
Áreas relacionadas:
Monitoramento de processos e recursos.
Observabilidade de eventos e alterações operacionais.
Detecção de anomalias.
Análise temporal.
Correlação de eventos.
2. Formiga --- inteligência distribuída
Referência para mecanismos de coordenação coletiva e otimização.
Áreas relacionadas:
Comunicação entre módulos e agentes.
Distribuição de tarefas.
Coordenação de componentes.
Otimização de rotas, incluindo referências a Ant Colony Optimization (ACO).
Reorganização operacional e tolerância a falhas.
3. Ariranha --- orquestração e defesa coordenada
Referência para mecanismos cooperativos de segurança e resposta operacional.
Áreas relacionadas:
Correlação de eventos de segurança.
Orquestração de respostas.
Contenção de incidentes.
Isolamento de subsistemas.
Recuperação operacional e rastreabilidade.
4. Falcão --- visão computacional e precisão analítica
Referência para percepção visual, reconhecimento e análise de informações.
Áreas relacionadas:
Aquisição e processamento de imagens.
Reconhecimento de padrões.
Detecção e classificação de objetos.
Análise de movimento.
Identificação de anomalias visuais.
BRAPI integra o contexto tecnológico do ecossistema associado ao trabalho com aquisição e processamento de informações visuais.
PHL --- Arquitetura de Estados Contínuos
A linha PHL está relacionada ao acompanhamento dos estados e das condições operacionais do sistema.
Entre os indicadores trabalhados no contexto do projeto estão:
Mitigações ativadas.
Estados degradados.
Isolamentos de subsistemas.
Eventos processados.
Integridade operacional.
Evolução temporal das métricas.
A interpretação de cada indicador depende de sua definição, fonte de dados e regra de cálculo documentadas na implementação correspondente.
Princípios de engenharia
Observabilidade: tornar o comportamento operacional inspecionável.
Modularidade: organizar responsabilidades em componentes especializados.
Coordenação: estabelecer comunicação e cooperação entre subsistemas.
Integridade: preservar consistência e rastreabilidade.
Adaptabilidade: responder a mudanças nas condições operacionais.
Autonomia arquitetural: investigar, compreender, desenvolver e evoluir mecanismos próprios.
Validação experimental: avaliar o comportamento por testes e evidências técnicas.
Método de trabalho
A engenharia do ATCS envolve atividades como:
Inspeção estrutural de sistemas.
Análise de componentes, dependências e fluxos de execução.
Investigação de mecanismos internos.
Modelagem de comportamentos e estados.
Implementação e integração de componentes.
Testes de integridade, desempenho e comportamento.
Documentação e evolução arquitetural.
Estado do projeto
O ATCS Framework é desenvolvido de forma independente e encontra-se em evolução.
Este repositório deve ser consultado para identificar o estado efetivo de cada componente. A presença de um domínio ou eixo neste README não significa, por si só, que todas as suas funcionalidades estejam implementadas ou validadas.
Implementações, métricas, resultados de testes e níveis de maturidade devem ser descritos junto aos respectivos códigos e evidências.
Documentação
A documentação técnica, os registros de implementação e os resultados de validação devem acompanhar a evolução do repositório.
Ao contribuir ou analisar o projeto, diferencie:
conceitos e fundamentos arquiteturais;
componentes efetivamente implementados;
funcionalidades em desenvolvimento;
resultados comprovados por testes.
Autor
River Feitosa
Autor e idealizador do ATCS Framework
Intelligence Global
ATCS Framework --- Engenharia de Sistemas, Inteligência Computacional e Autonomia Arquitetural.