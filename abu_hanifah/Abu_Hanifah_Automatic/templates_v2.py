"""Templates for Variasi 2: Ceramah Mufti Menk + Tulis Word + Subtitle Interaktif.

Opening sentence is FIXED as requested by user:
'Haritu ada nampak ada satu threads ni kata kalau nak fluent english in 2months
cuba konsisten setiap hari dengar ceramah mufti menk. Inshaa Allah boleh.'

All templates are paired with 2 images:
- variasi2a.png (Mobile mockup)
- variasi2b.png (Desktop player)
Length is strictly under 475 characters (Threads limit: 500 chars).
"""

from typing import Dict, List
from pathlib import Path

# Associated images for Variasi 2
VARIASI_2_IMAGES: List[str] = [
    str(Path(__file__).parent / "variasi2a.png"),
    str(Path(__file__).parent / "variasi2b.png"),
]

FIXED_HOOK: str = (
    "Haritu ada nampak ada satu threads ni kata kalau nak fluent english in 2months "
    "cuba konsisten setiap hari dengar ceramah mufti menk. Inshaa Allah boleh."
)

TEMPLATES_V2: List[Dict[str, str]] = [
    {
        "name": "tulis_word_versi_asal",
        "hook": FIXED_HOOK,
        "core": (
            "Tips dari saya, awak kena tulis setiap word yang baru tahu. Menulis tu buat word lekat dalam otak!\n\n"
            "Cuba pakai website saya, setiap video ada subtitle Malay & English. Siap boleh tengok terjemahan word "
            "terus dari subtitle macam dalam gambar. RM50 setahun je."
        ),
    },
    {
        "name": "fokus_klik_terus_subtitle",
        "hook": FIXED_HOOK,
        "core": (
            "Tapi tips saya, tulis word baru tu biar lekat kat minda.\n\n"
            "Boleh praktis kat website saya. Tak payah susah buka kamus, klik je perkataan kat subtitle video tu "
            "terus keluar maksud bahasa Melayu (rujuk gambar). Yuran cuma RM50 setahun penuh."
        ),
    },
    {
        "name": "transkrip_interaktif_lengkap",
        "hook": FIXED_HOOK,
        "core": (
            "Tips penting: tulis setiap perkataan baru yang didengar.\n\n"
            "Kat website saya, ada pemain video khas dengan transkrip interaktif & 2 subtitle serentak. "
            "Boleh terus tekan perkataan untuk tahu maksud macam tangkap layar di bawah. RM50 setahun."
        ),
    },
    {
        "name": "rutin_harian_mudah",
        "hook": FIXED_HOOK,
        "core": (
            "Kuncinya kena rajin tulis perkataan baru yang jumpa setiap hari biar lekat dalam memori.\n\n"
            "Cuba praktis di website kami. Setiap video ada subtitle dwibahasa dan terjemahan kosa kata segera "
            "(tengok gambar). Pelaburan ilmu cuma RM50 untuk setahun."
        ),
    },
    {
        "name": "dengar_sambil_catat_vocab",
        "hook": FIXED_HOOK,
        "core": (
            "Sambil dengar ceramah, tips terbaik adalah catat vocab baru supaya tak cepat lupa.\n\n"
            "Website saya bantu sediakan video dengan subtitle Malay + English & auto-terjemah perkataan "
            "atas skrin macam gambar ni. Akses 365 hari baru RM50."
        ),
    },
    {
        "name": "bina_kelancaran_2_bulan",
        "hook": FIXED_HOOK,
        "core": (
            "Untuk hasil padu dalam 2 bulan, tulis word baru yang dipelajari setiap hari.\n\n"
            "Dalam website saya, pengguna boleh dengar sambil rujuk video 2 subtitle dan klik maksud "
            "perkataan terus (tengok gambar). Yuran bajet rahmah RM50 setahun."
        ),
    },
    {
        "name": "telinga_minda_selari",
        "hook": FIXED_HOOK,
        "core": (
            "Dengar ceramah memang mantap, tapi lagi cepat lekat kalau kita tulis perkataan baru tu.\n\n"
            "Guna website saya untuk tonton video ber-subtitle dwibahasa. Siap ada pop-up maksud "
            "perkataan bila klik (rujuk gambar). Murah je, RM50 setahun."
        ),
    },
    {
        "name": "solusi_tak_perlu_kamus",
        "hook": FIXED_HOOK,
        "core": (
            "Nasihat saya, jangan lupa tulis setiap perkataan baru yang dipelajari biar kemas dalam otak.\n\n"
            "Website kami mudahkan proses ni: ada subtitle dwi-bahasa dan maksud perkataan terpapar "
            "terus bila ditekan (seperti dalam gambar). RM50 setahun."
        ),
    },
]
