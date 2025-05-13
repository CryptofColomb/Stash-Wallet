#!/bin/bash
set -e  # Hata olursa betik durur

# Hedef dizini temizle
rm -rf dist/stash-wallet/
mkdir -p dist/stash-wallet/

# Rust backend derlenip var mı kontrol et
if [ ! -f "core/target/release/core" ]; then
    echo "❌ Rust binary 'core/target/release/core' bulunamadı. Önce 'cargo build --release' çalıştırın."
    exit 1
fi

# Electron UI derlenip var mı kontrol et
if [ ! -d "ui/build" ]; then
    echo "❌ Electron UI henüz derlenmemiş. Önce 'cd ui && npm run build' çalıştırın."
    exit 1
fi

# Rust binary kopyala
cp -r core/target/release/core dist/stash-wallet/

# Electron UI build klasörünü kopyala
cp -r ui/build/* dist/stash-wallet/

# Diğer klasörleri kopyala
cp -r contracts dist/stash-wallet/
cp -r fiat-gateway dist/stash-wallet/
cp -r docs dist/stash-wallet/
cp -r LICENSE dist/stash-wallet/
cp -r Cargo.toml dist/stash-wallet/
cp -r package.json dist/stash-wallet/
cp -r README_*.md dist/stash-wallet/

# Rust binary iznini ayarla
chmod +x dist/stash-wallet/core

# ZIP dosyası oluştur
cd dist
zip -r ../stash.zip stash-wallet/

# Opsiyonel: ISO/IMG USB imajı oluştur (Linux/macOS)
if command -v mkisofs &> /dev/null; then
    mkisofs -J -r -o ../stash-wallet-usb.iso stash-wallet/
    echo "✅ ISO dosyası oluşturuldu: stash-wallet-usb.iso"
else
    echo "⚠️ mkisofs yüklü değil. ISO dosyası oluşturulamadı."
fi

echo "✅ stash.zip dosyası oluşturuldu ve tüm dosyalar kopyalandı."