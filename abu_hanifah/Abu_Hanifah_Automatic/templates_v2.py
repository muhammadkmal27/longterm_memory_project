"""Templates for Variasi 2: Ceramah Mufti Menk + Tulis Word + Subtitle Interaktif.

All templates are designed to be paired with:
- variasi2a.png (Mobile mockup)
- variasi2b.png (Desktop player)
Length is strictly under 460 characters (Threads limit: 500 chars).
"""

from typing import Dict, List
from pathlib import Path

# Associated images for Variasi 2
VARIASI_2_IMAGES: List[str] = [
    str(Path(__file__).parent / "variasi2a.png"),
    str(Path(__file__).parent / "variasi2b.png"),
]

TEMPLATES_V2: List[Dict[str, str]] = [
    {
        "name": "menk_viral_tip_asli",
        "hook": "Haritu ada nampak threads kata kalau nak fluent English in 2 months, konsisten dengar ceramah Mufti Menk.",
        "core": (
            "Tips dari saya: tulis setiap word baru yang awak jumpa. Menulis tu buat perkataan lekat dalam otak!\n\n"
            "Cuba pakai website saya. Dalam setiap video ada subtitle Malay & English, siap boleh klik tengok terjemahan perkataan terus dari subtitle (tengok gambar).\n\n"
            "Harga murah je, RM50 setahun."
        ),
    },
    {
        "name": "teknik_tulis_word_lekat",
        "hook": "Kenapa dengar ceramah Mufti Menk bagus tapi masih ramai susah nak ingat vocab?",
        "core": (
            "Sebab dengar saja tak cukup, kena tulis word baru tu biar lekat dalam otak!\n\n"
            "Website kami sediakan video ceramah dengan 2 subtitle (Melayu & English). Paling best, boleh terus tengok maksud perkataan atas skrin macam dalam gambar.\n\n"
            "Akses setahun penuh cuma RM50."
        ),
    },
    {
        "name": "interaktif_klik_subtitle",
        "hook": "Tengah tonton video English, tiba-tiba jumpa perkataan pelik tak tahu makna?",
        "core": (
            "Kat website saya, tak payah susah-susah buka kamus lain. Klik je perkataan pada subtitle tu, terus keluar terjemahan bahasa Melayu (rujuk gambar)!\n\n"
            "Sangat sesuai sambil dengar ceramah Mufti Menk untuk mantapkan vocab. Yuran RM50 setahun."
        ),
    },
    {
        "name": "cabaran_2_bulan_fluent",
        "hook": "Cabaran 2 bulan fasih English secara konsisten:",
        "core": (
            "1. Setiap hari dengar ceramah Mufti Menk.\n"
            "2. Tulis setiap perkataan baru yang dipelajari.\n"
            "3. Praktis guna website saya yang ada 2 subtitle serentak + maksud perkataan terus pada skrin (rujuk gambar).\n\n"
            "Pelaburan ilmu bajet rahmah, cuma RM50 setahun."
        ),
    },
    {
        "name": "sebutan_jelas_mufti_menk",
        "hook": "Antara cara paling best latih telinga faham English adalah dengar ceramah Mufti Menk.",
        "core": (
            "Sebutan beliau sangat jelas dan tenang. Bila digabungkan dengan teknik menulis vocab baru, memang cepat lekat.\n\n"
            "Dalam website kami, semua video ada subtitle dwibahasa & terjemahan interaktif macam dalam tangkap layar ni. Akses cuma RM50/tahun."
        ),
    },
    {
        "name": "pengisian_rohani_sambil_belajar",
        "hook": "Belajar English sambil dapat pengisian rohani dan tazkirah bermanfaat.",
        "core": (
            "Sambil dengar ceramah Mufti Menk, sambil tu latih vocab harian. Kuncinya: tulis perkataan yang baru dipelajari.\n\n"
            "Website saya bantu dengan paparan video 2 subtitle (Malay & English) siap maksud word automatik (tengok gambar). RM50 je setahun."
        ),
    },
    {
        "name": "panduan_visual_gambar",
        "hook": "Cuba tengok 2 gambar di bawah ni. Macam ni rupa cara kami latih vocabulary:",
        "core": (
            "Video ceramah (macam Mufti Menk) dipaparkan bersama subtitle dwibahasa. Bila jumpa perkataan asing, maksudnya terus muncul tanpa pening kepala.\n\n"
            "Tulis dan faham perkataan baru setiap hari. Yuran cuma RM50 untuk 365 hari akses."
        ),
    },
    {
        "name": "nilai_rahmah_transkrip_interaktif",
        "hook": "Ramai nak praktis English dengar ceramah Mufti Menk tapi tersekat sebab vocab terhad.",
        "core": (
            "Sebab tu dalam website saya, kami sediakan transkrip interaktif dengan terjemahan maksud perkataan terus dari video (macam gambar dilampirkan).\n\n"
            "Tulis word baru, hafal dan faham. Akses setahun baru RM50."
        ),
    },
]

