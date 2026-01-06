#!/bin/bash
# Lazarus - 벵갈어(bn) + 터키어(tr) 추가 스크립트
# 사용법: bash add_bn_tr.sh

cd ~/Documents/lazarus

echo "🌍 벵갈어 + 터키어 추가 시작..."

# ============================================
# 1. 언어 파일 복사 (bn.rs, tr.rs)
# ============================================
echo "📁 1/4: 언어 파일 복사..."

# bn.rs와 tr.rs는 별도로 복사해야 함
# 이 스크립트 실행 전에 bn.rs, tr.rs를 src/i18n/에 복사하세요

if [ ! -f src/i18n/bn.rs ]; then
    echo "❌ src/i18n/bn.rs 파일이 없습니다!"
    echo "   먼저 bn.rs 파일을 src/i18n/에 복사하세요."
    exit 1
fi

if [ ! -f src/i18n/tr.rs ]; then
    echo "❌ src/i18n/tr.rs 파일이 없습니다!"
    echo "   먼저 tr.rs 파일을 src/i18n/에 복사하세요."
    exit 1
fi

echo "   ✅ bn.rs, tr.rs 확인됨"

# ============================================
# 2. mod.rs 업데이트
# ============================================
echo "📝 2/4: mod.rs 업데이트..."

# mod 선언 추가 (mod yue; 다음에)
sed -i '/^mod yue;/a mod bn;\nmod tr;' src/i18n/mod.rs

# Lang enum에 Bn, Tr 추가 (Yue, 다음에)
sed -i '/Yue,.*광둥어/a\    Bn,   \/\/ 벵갈어\n    Tr,   \/\/ 터키어' src/i18n/mod.rs

# from_accept_language에 bn, tr 추가 (yue 체크 전에)
sed -i '/starts_with("yue")/i\        } else if header.starts_with("bn") || header.contains("bn-") || header.contains("bn,") {\n            Self::Bn\n        } else if header.starts_with("tr") || header.contains("tr-") || header.contains("tr,") {\n            Self::Tr' src/i18n/mod.rs

# code() 함수에 bn, tr 추가 (Yue 다음에)
sed -i '/Self::Yue => "yue",/a\            Self::Bn => "bn",\n            Self::Tr => "tr",' src/i18n/mod.rs

# is_rtl() 함수는 그대로 (bn, tr은 LTR)

# get_translations에 Bn, Tr 추가
sed -i '/Lang::Yue => yue::translations(),/a\            Lang::Bn => bn::translations(),\n            Lang::Tr => tr::translations(),' src/i18n/mod.rs

echo "   ✅ mod.rs 업데이트 완료"

# ============================================
# 3. lang.rs 업데이트 (핸들러)
# ============================================
echo "📝 3/4: lang.rs 업데이트..."

# "yue" => Lang::Yue, 다음에 bn, tr 추가
sed -i '/"yue" => Lang::Yue,/a\        "bn" => Lang::Bn,\n        "tr" => Lang::Tr,' src/web/handlers/lang.rs

echo "   ✅ lang.rs 업데이트 완료"

# ============================================
# 4. settings.html 드롭다운 업데이트
# ============================================
echo "📝 4/4: settings.html 드롭다운 업데이트..."

# Tier 1에 벵갈어 추가 (Hindi 다음에)
sed -i '/<option value="hi">🇮🇳 हिन्दी<\/option>/a\                <option value="bn">🇧🇩 বাংলা<\/option>' templates/settings.html

# Tier 2에 터키어 추가 (Russian 다음에)
sed -i '/<option value="ru">🇷🇺 Русский<\/option>/a\                <option value="tr">🇹🇷 Türkçe<\/option>' templates/settings.html

echo "   ✅ settings.html 업데이트 완료"

# ============================================
# 완료
# ============================================
echo ""
echo "✅ 벵갈어 + 터키어 추가 완료!"
echo ""
echo "📋 추가된 언어:"
echo "   🇧🇩 বাংলা (bn) - Tier 1"
echo "   🇹🇷 Türkçe (tr) - Tier 2"
echo ""
echo "🔧 이제 빌드하세요:"
echo "   cargo build && cargo run"
echo ""
echo "📊 총 지원 언어: 16개"
