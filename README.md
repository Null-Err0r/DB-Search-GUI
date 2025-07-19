# 🔍 DB-Search GUI (پشتیبانی از متن فارسی)
جستجو در دیتاهای متنی خاص با رابط گرافیکی و پشتیبانی از زبان فارسی


ابزاری سریع و گرافیکی برای جستجوی عبارت یا الگوی Regex در میان فایل‌های متنی (مثل `.txt`, `.log`, `.docx`, `.json`, ...) با استفاده از زبان Rust.  
- پشتیبانی از mmap برای سرعت بالا  
- موازی‌سازی با rayon  
- رابط گرافیکی با egui  
- انتخاب پوشه با File Picker  
- ذخیره نتایج در فایل `results.txt`  
- نمایش نتایج در جدول قابل پیمایش

---



A fast and lightweight GUI tool written in Rust for searching keywords or regex patterns across large text-based files like `.txt`, `.log`, `.json`, etc.

- Uses `mmap` for memory-efficient file reading  
- Multi-threaded with `rayon`  
- Built with `egui` for a modern GUI  
- Folder selection via File Picker  
- Saves results to `results.txt`  
- Live result preview in scrollable view

---

## ⚙️ Build Instructions | دستور ساخت

```bash

git clone https://github.com/your-username/rust-text-search
cd rust-text-search

cargo run --release

```

---

## 📁 پشتیبانی از پسوندها | Supported Extensions

```
.txt .log .md .csv .json .xml .html .eml .rtf .odt .ods .docx .xlsx .java .kt .epub
```

---


## 📄 License | لایسنس

This project is licensed under the [MIT License](LICENSE).  
این پروژه تحت لایسنس MIT منتشر شده است.


<div align="center">
  <br> </br>
  <img src="https://ghvc.kabelkultur.se/?username=null-err0r&repository=DB-Search-GUI&label=DB-Search-GUI%20Views⁮⁮"/>
</div>
