# GitHub Personal Access Token - Quick Setup

## محتاج Token عشان ترفع الكود لـ GitHub

### الخطوات (دقيقة واحدة):

1. **اذهب لـ**: https://github.com/settings/tokens/new

2. **املأ البيانات:**
   - **Note**: `Nova Container System`
   - **Expiration**: `90 days`
   - **Select scopes**: اختار `repo` ✅ (كل الـ repo permissions)

3. **اضغط** "Generate token"

4. **انسخ الـ token** (هيظهر مرة واحدة بس!)

5. **نفذ الأمر:**
```bash
cd /Users/mahmoudabdelkawy/.gemini/antigravity/scratch/nova_container_system
git push -u origin main
```

**لما يطلب password، استخدم الـ token!**

---

## بعد ما ترفع الكود:

### الخطوة التالية: Create Release
```bash
git tag -a v0.1.0-beta -m "Beta Release"
git push origin v0.1.0-beta
```

### ثم على GitHub:
1. اذهب لـ: https://github.com/m7moud2/nova-container-system/releases/new
2. Tag: `v0.1.0-beta`
3. Title: `Nova v0.1.0 Beta`
4. Description: انسخ من `RELEASE_NOTES.md`
5. Upload: `target/release/nova_container_system`
6. Publish!

---

**بعد كده قولي "تمام" وأكمل معاك الموقع!** 🚀
