# Aegis Privacy Shield 🛡️

![Aegis Status](https://img.shields.io/badge/Status-Stable-emerald) ![Security](https://img.shields.io/badge/Security-Hardened-blueviolet) ![License](https://img.shields.io/badge/License-MIT-blue)

---

## 🇧🇷 Resumo (Português)
**O que é esse app?**
O **Aegis** é uma ferramenta de privacidade completa para Linux. Ele redireciona **toda** a sua internet para a rede Tor, garantindo que ninguém (nem sites, nem seu provedor) saiba quem você é ou onde você está.

**Para quem é?**
Para qualquer pessoa que queira navegar na internet sem ser rastreada. Não precisa ser hacker! Basta clicar no botão gigante de ligar.

**O que ele faz por você?**
*   **Esconde seu IP**: Sites vão achar que você está na Suíça, Alemanha, etc.
*   **Muda seu "DNA" (MAC/Hostname)**: Troca a "impressão digital" do seu computador para evitar rastreamento físico.
*   **Limpa seus rastros**: Um botão para apagar a memória RAM e logs do sistema.
*   **Bloqueia vazamentos**: Impede conexões inseguras que poderiam revelar sua identidade real.

---

## 🇺🇸 Summary (English)
**What is this app?**
**Aegis** is a comprehensive privacy tool for Linux. It transparently routes **all** your internet traffic through the Tor network, ensuring that neither websites nor your ISP can track your identity or location.

**Who is it for?**
For anyone who wants to browse the web anonymously. You don't need to be a tech expert! Just click the big power button.

**What does it do?**
*   **Hides your IP**: Websites will think you are connecting from random countries.
*   **Changes your "DNA" (MAC/Hostname)**: Randomizes system identifiers to prevent hardware fingerprinting.
*   **Wipes Traces**: Features to clear RAM and system logs instantly.
*   **Leak Protection**: Blocks insecure connections (like IPv6 leaks) automatically.

---

## 🇧🇷 Funcionalidades (Português)

### 🔒 Privacidade Central
*   **Proxy Transparente**: Diferente de plugins de navegador, o Aegis força *tudo* (Atualizações, Chats, Serviços de Fundo) através do Tor.
*   **True Killswitch**: O firewall bloqueia tudo por padrão (`DROP`). Se o Tor não estiver ativo, nada sai do seu computador.
*   **Criptografia DNS**: Força todas as requisições de sites através da porta segura 5353 do Tor.

### 🕵️ Módulos Furtivos (Stealth)
*   **MAC Spoofing**: Randomiza o endereço físico (MAC) da sua placa de rede com um clique. Inclui botão de "Reset" para restaurar o original.
*   **Troca de Hostname**: Renomeia seu computador (ex: de `fabio-pc` para `anon-8392`) para confundir scanners de rede local.
*   **Horário UTC**: Ajusta o relógio do sistema para UTC, combatendo a identificação por fuso horário (fingerprinting).

### 🧹 Forensics (Limpeza)
*   **Limpeza de RAM**: Apaga dados em cache na memória RAM para prevenir ataques "Cold Boot" ou análise forense.
*   **Limpeza de Logs**: Trunca logs sensíveis do sistema (`syslog`, `auth.log`, `kern.log`) para remover o histórico das suas ações.

### 📊 Dashboard Moderno
*   **Gráfico em Tempo Real**: Visualiza velocidades de upload/download criptografadas via Tor.
*   **Inspetor de Identidade**: Mostra seu status IPv6 (Bloqueado/Seguro), Endereço MAC e IP Público.
*   **Log de Atividade**: Veja exatamente o que o Aegis está fazendo (regras de firewall, reinícios de serviço, etc).

### ⚔️ Defesa Ativa (Active Defense) [NOVO]
*   **Detector de Rastreamento (GeoIP):** Monitora conexões de entrada de países específicos (ex: Israel) conhecidos por exportar tecnologias de vigilância.
*   **Honeypot de Resposta:** Se um rastreador for detectado, o Aegis aceita a conexão momentaneamente, envia uma mensagem personalizada (configurável pelo usuário) e derruba a conexão. O rastreador recebe sua mensagem ao invés dos seus dados.
*   **Alertas de Segurança**: Notifica visualmente no painel quando tentativas de rastreamento são bloqueadas.

---

## 🇺🇸 Features (English)

### 🔒 Core Privacy
*   **Transparent Proxy**: Unlike browser plugins, Aegis forces *everything* (Update managers, Chat apps, Background services) through Tor.
*   **True Killswitch**: The firewall defaults to `DROP`. If Tor isn't active, no internet traffic leaves your computer.
*   **DNS Encryption**: Forces all domain requests through Tor's secure port 5353.

### 🕵️ Stealth Modules
*   **MAC Spoofing**: Randomizes your Network Card's physical address (MAC) with a single click. Includes a "Reset" button to restore the original.
*   **Hostname Changer**: Renames your computer (e.g., from `fabio-pc` to `anon-8392`) to confuse local network scanners.
*   **UTC Timezone**: Sets your system clock to UTC time to fight browser fingerprinting (which uses timezone to guess location).

### 🧹 Forensics
*   **RAM Wiper**: Clears cached data in RAM to prevent "Cold Boot" attacks or memory forensic analysis.
*   **Log Cleaner**: Truncates sensitive system logs (`syslog`, `auth.log`, `kern.log`) to remove history of your actions.

### 📊 Modern Dashboard
*   **Real-Time Graph**: Visualizes upload/download speeds encrypted via Tor.
*   **Identity Inspector**: Shows your current IPv6 status (Blocked/Secure), MAC Address, and Public IP.
*   **Live Activity Log**: Watch exactly what Aegis is doing in the background (iptables rules, service restarts, etc).

### ⚔️ Active Defense [NEW]
*   **Tracking Detector (GeoIP):** Monitors incoming connections from specific countries (e.g., Israel) known for exporting surveillance tech.
*   **Response Honeypot:** If a tracker is detected, Aegis momentarily accepts the connection, sends a custom message (configurable by you), and drops the connection. The tracker gets your message instead of your data.
*   **Security Alerts:** Visually notifies you on the dashboard when tracking attempts are blocked.

## 🛠️ Installation

```bash
# 1. Clone & Setup
git clone https://github.com/your-repo/aegis.git
cd aegis
chmod +x setup.sh
sudo ./setup.sh

# 2. Run
chmod +x run.sh
./run.sh
```

**Note**: Aegis requires `sudo` privileges to modify network interfaces and firewall rules.

## ☕ Support the Developer

If Aegis helps keep you safe, consider supporting the project!

*   **Buy Me a Coffee**: [buymeacoffee.com/belydev](https://buymeacoffee.com/belydev)
*   **Monero (XMR)**: `466KtH3FTWFYJ2xN9McVzzPnNXZf4GGZr2AQ9eQP6RfuYzd2WkTrckf4ySZF8SsdQQNiyWToG8mTP1DaQfsGTd5p2MkMZTN`
*   **ZCash (ZEC)**: `u1zzjp0gh9ms5wcfd5uqsj47jjad7qufqm4pugqw0l96h0374zu3pfn0we0v2g88p0apap9y38kj5dasjcnl6sll7psjfx7g763ymrd57t`
*   **Bitcoin (BTC)**: `bc1quz3lk0s2wzcpycz545dkzcn5lqyct9z60maafp`

<p align="center">
  <img src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExM3R4eGxpamI5aW14eHB4eXJ4eXJ4eXJ4eXJ4eXJ4eXJ4ZSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/L59z1m8V5V5A/giphy.gif" width="50" />
  <br>
  <i>Built with Rust + Tauri + React</i>
</p>
