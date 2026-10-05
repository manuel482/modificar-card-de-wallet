# Aplicación Prueba Beta 🎴

> Aplicación experimental para Windows orientada a la personalización visual de tarjetas de Apple Wallet y temas de código de bloqueo en iOS compatibles.

**Versión:** Beta · **Autor de esta adaptación:** Manuel  
**Plataforma:** Windows 10/11 (64 bits) · **Tecnología:** Rust + egui/eframe

---

## 📌 Descripción

**Aplicación Prueba Beta** es una adaptación experimental del proyecto AirCard para Windows. Permite trabajar con determinados recursos visuales de Apple Wallet desde una computadora Windows conectada a un iPhone compatible.

La aplicación mantiene el funcionamiento técnico del proyecto base, pero esta versión utiliza una identidad visual personalizada para fines de prueba y desarrollo.

---

## ✨ Funciones principales

- 🎨 **Personalización visual de tarjetas:** permite seleccionar imágenes personalizadas para tarjetas compatibles de Apple Wallet.
- 🔎 **Detección de tarjetas:** identifica el hash de la tarjeta mediante eventos del dispositivo conectado.
- 💾 **Copia de seguridad:** conserva una copia local del diseño original antes de aplicar cambios.
- ♻️ **Restauración:** permite restaurar el diseño original previamente respaldado.
- 🔢 **Temas de código de bloqueo:** compatibilidad con paquetes `.passthm`.
- 🔌 **Conexión USB:** comunicación directa con dispositivos Apple compatibles.
- 📶 **Conexión Wi-Fi:** admite dispositivos previamente emparejados y configurados para sincronización Wi-Fi.
- 🪟 **Interfaz nativa para Windows:** desarrollada con Rust, egui y eframe.
- 🚫 **Sin jailbreak:** no requiere modificar las particiones del sistema.

---

## 🖥️ Requisitos

- Windows 10 u 11 de 64 bits.
- Apple Mobile Device Support / iTunes de 64 bits.
- iPhone compatible.
- Cable Lightning o USB-C para el emparejamiento inicial.
- Para utilizar Wi-Fi, el iPhone y la PC deben estar en la misma red local y tener habilitada la sincronización por Wi-Fi.

---

## 🚀 Compilar y ejecutar

Primero instala el toolchain estable de Rust para Windows.

```powershell
git clone https://github.com/manuel482/modificar-card-de-wallet.git
cd modificar-card-de-wallet

cargo test
cargo run
```

Para generar una compilación optimizada:

```powershell
cargo build --release
```

El ejecutable generado actualmente se encuentra en:

```text
target\release\aircard.exe
```

Al abrirlo, la interfaz se muestra como **Aplicación Prueba Beta**.

---

## 📱 Uso básico con Apple Wallet

1. Conecta y desbloquea el iPhone.
2. Abre **Aplicación Prueba Beta**.
3. Entra en la sección **Wallet**.
4. Inicia el escaneo.
5. Abre Wallet en el iPhone y selecciona la tarjeta que deseas identificar.
6. Detén el escaneo cuando la aplicación detecte la tarjeta.
7. Selecciona una imagen compatible.
8. Revisa la vista previa antes de aplicar cualquier cambio.
9. Utiliza la función de restauración si deseas recuperar el diseño original respaldado.

> [!IMPORTANT]
> Utiliza únicamente esta aplicación en dispositivos propios y conserva siempre una copia de seguridad antes de realizar modificaciones.

---

## 🔐 Temas de código de bloqueo

La aplicación también conserva la compatibilidad del proyecto base con paquetes `.passthm`. La disponibilidad puede variar según la versión de iOS y los recursos de caché utilizados por el sistema.

---

## 🧪 Estado del proyecto

Este repositorio corresponde a una **versión beta de prueba**. Puede contener errores y su comportamiento puede variar entre versiones de iOS.

---

## 👤 Adaptación

**Manuel**  
Repositorio: [manuel482/modificar-card-de-wallet](https://github.com/manuel482/modificar-card-de-wallet)

---

## 🙏 Proyecto base y créditos

Esta adaptación deriva de **AirCard-Windows** y conserva reconocimiento a los desarrolladores e investigadores del proyecto original:

- [Lumid-Off](https://github.com/Lumid-Off) — AirCard-Windows / implementación nativa para Windows en Rust.
- [mak5er](https://github.com/mak5er) — aplicación original para macOS e investigación relacionada.
- [AirLift](https://github.com/0xjohnnydev/airlift) por [0xjohnnydev](https://github.com/0xjohnnydev) — investigación y prueba de concepto relacionada con AirTraffic/Airlift.
- Formato de temas inspirado en [Cowabunga](https://github.com/leminlimez/Cowabunga) y [Nugget](https://github.com/leminlimez/Nugget).

Proyecto original: [Lumid-Off/AirCard-Windows](https://github.com/Lumid-Off/AirCard-Windows)

---

## 📄 Licencia

Distribuido bajo la licencia MIT incluida en el archivo `LICENSE`.
