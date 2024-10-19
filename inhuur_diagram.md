```mermaid
flowchart LR
state_no_zzp["☠ NO ZZP"]

klant_belastingdienst_ict["Belastingdienst ICT"]
klant_belastingdienst_non_ict["Belastingdienst Non-ICT"]
klant_capgemini_nederland_["Capgemini Nederland "]
klant_cegeka_nederland["Cegeka Nederland"]
klant_ciz["CIZ"]
klant_college_ter_beoordeling_van_geneesmiddelen["College ter Beoordeling van Geneesmiddelen"]
klant_dienst_justitiële_inrichtingen["Dienst Justitiële Inrichtingen"]
klant_dpg_media["DPG Media"]
klant_enexis["Enexis"]
klant_hr_ministerie_van_infrastructuur_en_waterstaat["HR - Ministerie van Infrastructuur en Waterstaat"]
klant_hr_rijkswaterstaat["HR - Rijkswaterstaat"]
klant_koninklijk_nederlands_meteorologisch_instituut["Koninklijk Nederlands Meteorologisch Instituut"]
klant_kpn["KPN"]
klant_ministerie_van_defensie["Ministerie van Defensie"]
klant_ministerie_van_infrastructuur_en_waterstaat["Ministerie van Infrastructuur en Waterstaat"]
klant_ministerie_van_volksgezondheid_welzijn_en_sport["Ministerie van Volksgezondheid, Welzijn en Sport"]
klant_mobilis["Mobilis"]
klant_ndw["NDW"]
klant_npo["NPO"]
klant_ns["NS"]
klant_pgb_pensioendiensten["PGB Pensioendiensten"]
klant_pggm["PGGM"]
klant_provincie_zuid_holland["Provincie Zuid-Holland"]
klant_rabobank["Rabobank"]
klant_rabobank["Rabobank"]
klant_rijksinstituut_voor_de_volksgezondheid_en_milieu["Rijksinstituut voor de Volksgezondheid en Milieu"]
klant_rijkswaterstaat["Rijkswaterstaat"]
klant_sociale_verzekeringsbank["Sociale Verzekeringsbank"]
klant_stedin_netbeheer["Stedin Netbeheer"]
klant_tennet["TenneT"]
klant_umc_utrecht["UMC Utrecht"]
klant_unilever["Unilever"]
klant_vng_realisatie["VNG Realisatie"]

recruiter_citrus_it{{"Citrus IT"}}
recruiter_vibe_group{{"Vibe group"}}
recruiter_sevenstars{{"Sevenstars"}}
recruiter_yellowfriday{{"Yellow Friday"}}

broker_yacht(["Yacht"])
broker_js_consultancy(["JS Consultancy"])
broker_magnit(["Magnit"])
broker_ctm_solution(["CTM solution"])
broker_headfirst(["Headfirst"])

das_freelance_nl(("Freelance.nl"))
das_freep(("Freep.nl"))
das_headfirst_select((Headfirst Select))
das_human("🫂 Direct netwerken")
das_magnit(("Magnit portal"))
das_negometrix(("Negometrix 3"))
das_ns(("Inhuur NS"))
das_sentior((Sentior))
das_staffd((Staffd))
das_striive(["Striive"])
das_unilever_talentpool(("Unilever freelancers talent-pool"))

%% Edges: we werken van klant de diepte in.
klant_belastingdienst_ict --> broker_capgemini
klant_belastingdienst_ict --> broker_headfirst
klant_belastingdienst_non_ict --> broker_headfirst
klant_capgemini_nederland_ --> broker_headfirst
klant_cegeka_nederland --> broker_headfirst
klant_ciz --> broker_headfirst
klant_college_ter_beoordeling_van_geneesmiddelen --> broker_headfirst
klant_dienst_justitiële_inrichtingen --> broker_headfirst
klant_dpg_media --> das_human
klant_enexis --> recruiter_citrus_it
klant_enexis --> recruiter_vibe_group
klant_hr_ministerie_van_infrastructuur_en_waterstaat --> broker_headfirst
klant_hr_rijkswaterstaat --> broker_headfirst
klant_koninklijk_nederlands_meteorologisch_instituut --> broker_headfirst
klant_kpn --> broker_yacht
klant_kpn --> das_staffd
klant_ministerie_van_defensie --> broker_headfirst
klant_ministerie_van_infrastructuur_en_waterstaat --> broker_headfirst
klant_ministerie_van_justitie_en_veiligheid --> recruiter_sevenstars
klant_ministerie_van_volksgezondheid_welzijn_en_sport --> broker_headfirst
klant_mobilis --> broker_magnit
klant_ndw --> broker_headfirst
klant_npo -- misschien --> das_negometrix
klant_ns --> das_ns
klant_pgb_pensioendiensten --> broker_headfirst
klant_pggm --> broker_headfirst
klant_provincie_zuid_holland --> broker_js_consultancy
klant_rabobank --> broker_headfirst
klant_rabobank --> broker_magnit
klant_rabobank --> state_no_zzp
klant_rijksinstituut_voor_de_volksgezondheid_en_milieu --> broker_headfirst
klant_rijkswaterstaat --> broker_headfirst
klant_sociale_verzekeringsbank --> broker_headfirst
klant_stedin_netbeheer --> broker_headfirst
klant_tennet --> broker_magnit
klant_umc_utrecht --> broker_headfirst
klant_unilever --> das_unilever_talentpool
klant_vattenfall --> broker_headfirst 
klant_vng_realisatie --> broker_ctm_solution

broker_ctm_solution --> das_freep
broker_js_consultancy --> das_sentior
broker_magnit --> das_magnit
broker_headfirst --> das_headfirst_select
broker_capgemini --> recruiter_yellowfriday

das_headfirst_select --> das_sentior

recruiter_vibe_group --> das_sentior
recruiter_vibe_group --> das_freelance_nl

%% Links for nodes
click klant_enexis "https://www.enexis.nl/" "https://www.enexis.nl/"
click klant_kpn "https://www.kpn.com/" "https://www.kpn.com/"
click klant_mobilis "https://www.mobilis.nl/" "https://www.mobilis.nl/"
click klant_npo "https://inhuurkaart.nl/detail-info/efd3c2dd/info/" "https://inhuurkaart.nl/detail-info/efd3c2dd/info/"
click klant_provincie_zuid_holland "https://werkenvoor.zuid-holland.nl/" "https://werkenvoor.zuid-holland.nl/"
click klant_vng_realisatie "https://vng.nl/" "https://vng.nl/"

click das_freelance_nl "https://www.freelance.nl/" "https://www.freelance.nl/"
click das_headfirst "https://platform.select.hr/" "https://platform.select.hr/"
click das_magnit "https://portal.magnitglobal.com/" "https://portal.magnitglobal.com/"
click das_negometrix "https://platform.negometrix.com/" "https://platform.negometrix.com/"
click das_ns "https://inhuur-ns.my.site.com/" "https://inhuur-ns.my.site.com/"
click das_sentior "https://sentior.com/" "https://sentior.com/"
click das_staffd "https://www.staffd.co/" "https://www.staffd.co/"
click das_unilever_talentpool "https://unileverfreelancers.talent-pool.com/" "https://unileverfreelancers.talent-pool.com/"

click recruiter_sevenstars "https://sevenstars.nl/" "https://sevenstars.nl/"
click recruiter_vibegruop "https://vibegroup.com/onze-merken/" "https://vibegroup.com/onze-merken/"

click broker_headfirst "https://www.headfirst.nl/zelfstandig-professionals/" "https://www.headfirst.nl/zelfstandig-professionals/"
click broker_js_consultancy "https://www.jsconsultancy.nl/professionals/interim/" "https://www.jsconsultancy.nl/professionals/interim/"
click broker_magnit "https://magnitglobal.com/nl" "https://magnitglobal.com/nl"
click broker_tergos "https://www.tergos.nl/vacatures" "https://www.tergos.nl/vacatures"
click broker_yacht "https://www.yacht.nl/" "https://www.yacht.nl/"
```