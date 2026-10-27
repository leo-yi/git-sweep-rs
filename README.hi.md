# git-sweep-rs

[English](README.en.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [हिन्दी](README.hi.md) · [Français](README.fr.md) · [Русский](README.ru.md)

> ⚠️ **चेतावनी: उपयोग करने से पहले अपने रिपोजिटरी का बैकअप लें**
>
> कृपया इस टूल का उपयोग करने से पहले अपना रिपोजिटरी बैकअप लें (उदाहरण के लिए, महत्वपूर्ण लोकल ब्रांच को रिमोट पर पुश करें या रिपोजिटरी का बैकअप बनाएं)। यह टूल ब्रांचेस को हटाता है और हटाना अस्थायी रूप से हटाना है। पहली बार उपयोग के लिए `git-sweep-rs preview` चलाकर देखें कि कौन-सी ब्रांच हटेंगी, और पुष्टि के बाद ही `cleanup` या `cleanup-local` चलाएं। पुष्टि को छोड़ने के लिए `--force` का इस्तेमाल करें (सावधानी से)।

Rust में लिखा गया एक Git ब्रांच क्लीनअप टूल जो रिमोट और लोकल दोनों, मुख्य ब्रांच में मर्ज हो चुकी ब्रांचेस को जल्दी हटाने में मदद करता है।

## ✨ फीचर्स

- 🚀 उच्च प्रदर्शन: गति के लिए Rust में लिखा गया
- 🔍 सुरक्षित पूर्वावलोकन: हटाने से पहले ब्रांचेस को पूर्वावलोकन करें
- 🌐 रिमोट क्लीनअप: मुख्य ब्रांच में मर्ज हुई रिमोट ब्रांचेस हटाएं
- 💻 लोकल क्लीनअप: मुख्य ब्रांच में मर्ज हुई लोकल ब्रांचेस हटाएं
- 🎯 लचीला: मुख्य ब्रांच नाम और स्किप सूची को कॉन्फ़िगर करें
- ✅ इंटरैक्टिव कन्फर्मेशन: हटाने से पहले पुष्टि माँगता है (`--force` से छोड़ें)

## 📦 इंस्टॉलेशन

### Homebrew (macOS / Linux)

```bash
brew install leo-yi/tap/git-sweep-rs
```

### सोर्स से बिल्ड करें

सुनिश्चित करें कि आपके पास [Rust](https://www.rust-lang.org/tools/install) इंस्टॉल है, फिर:

```bash
git clone https://github.com/leo-yi/git-sweep-rs
cd git-sweep-rs
cargo build --release
```

### सिस्टम में इंस्टॉल करें

**विज्ञप्ति 1: cargo install का उपयोग करें**

```bash
cargo install git-sweep-rs
```

**विकल्प 2: मैनुअल कॉपी**

```bash
sudo cp target/release/git-sweep-rs /usr/local/bin/
```

## 🚀 उपयोग

### रिमोट ब्रांचेस

#### हटाने के लिए रिमोट ब्रांचेस का पूर्वावलोकन

```bash
git-sweep-rs preview --origin origin --master master
```

#### रिमोट ब्रांचेस हटाएँ

```bash
# हटाने से पहले पुष्टि पूछेगा
git-sweep-rs cleanup --origin origin --master master

# पुष्टि के बिना फोर्स हटाएँ
git-sweep-rs cleanup --force --origin origin --master master
```

### लोकल ब्रांचेस

#### हटाने के लिए लोकल ब्रांचेस का पूर्वावलोकन

```bash
git-sweep-rs preview-local --master master
```

#### लोकल ब्रांचेस हटाएँ

```bash
# हटाने से पहले पुष्टि पूछेगा
git-sweep-rs cleanup-local --master master

# पुष्टि के बिना फोर्स हटाएँ
git-sweep-rs cleanup-local --force --master master
```

## 📖 कमांड्स

| कमांड | विवरण | लक्ष्य |
|------:|:------|:------|
| `preview` | हटाने के लिए रिमोट ब्रांचेस का पूर्वावलोकन | रिमोट ब्रांचेस |
| `cleanup` | मर्ज हुई रिमोट ब्रांचेस हटाएँ | रिमोट ब्रांचेस |
| `preview-local` | हटाने के लिए लोकल ब्रांचेस का पूर्वावलोकन | लोकल ब्रांचेस |
| `cleanup-local` | मर्ज हुई लोकल ब्रांचेस हटाएँ | लोकल ब्रांचेस |

### सामान्य विकल्प

- `--origin <ORIGIN>`: रिमोट का नाम (डिफ़ॉल्ट: `origin`)
- `--master <MASTER>`: मुख्य ब्रांच का नाम (डिफ़ॉल्ट: `master`)
- `--nofetch`: रिमोट से अपडेट न लाएँ
- `--skip <BRANCHES>`: स्किप करने के लिए कॉमा-सेपरेटेड ब्रांच सूची
- `--force`: पुष्टि के बिना फोर्स हटाएँ (केवल cleanup)

## 💡 उदाहरण

### उदाहरण 1: रिमोट ब्रांचेस साफ़ करें

```bash
# 1. पहले पूर्वावलोकन करें
git-sweep-rs preview --origin origin --master main

# 2. पुष्टि के बाद हटाएँ
git-sweep-rs cleanup --origin origin --master main
```

### उदाहरण 2: लोकल ब्रांचेस साफ़ करें

```bash
# 1. पहले पूर्वावलोकन करें
git-sweep-rs preview-local --master main

# 2. पुष्टि के बाद हटाएँ
git-sweep-rs cleanup-local --master main
```

### उदाहरण 3: कुछ ब्रांच स्किप करें

```bash
# develop और staging को स्किप करें
git-sweep-rs preview --master main --skip "develop,staging"
git-sweep-rs cleanup --master main --skip "develop,staging"
```

### उदाहरण 4: रिमोट अपडेट न लें

```bash
# लोकल कैश्ड रिमोट जानकारी का उपयोग करें
git-sweep-rs preview --nofetch
git-sweep-rs cleanup --nofetch
```

## ⚠️ नोट्स

1. रिमोट ब्रांचेस हटाना विनाशकारी हो सकता है
   - पहले `preview` का उपयोग करें
   - हटाये गये रिमोट ब्रांचेस आसानी से पुनर्स्थापित नहीं हो सकते
   - अन्य सहयोगियों को `git fetch --prune` चलाना चाहिए

2. लोकल ब्रांचेस हटाना
   - सुरक्षित हटाने के लिए `git branch -d` का उपयोग करें
   - अनमर्ज्ड ब्रांचेस के लिए `git branch -D` का उपयोग करें

3. मुख्य ब्रांच का नाम
   - कई रिपोजिटरी `main` का उपयोग करते हैं
   - अपनी रीक से मेल खाने के लिए `--master main` का उपयोग करें

## 🤝 योगदान

Issue और PR स्वागत हैं!

## 📝 लाइसेंस

MIT License

## 🙏 आभार

प्रेरणा: Python परियोजना [git-sweep](https://github.com/arc90/git-sweep).

## 📮 फीडबैक

समस्याएँ या सुझाव के लिए:

- [Issue](https://github.com/leo-yi/git-sweep-rs/issues)
- [Pull Request](https://github.com/leo-yi/git-sweep-rs/pulls)

---

⭐ कृपया प्रोजेक्ट को स्टार करें यदि यह आपके लिए उपयोगी है!

