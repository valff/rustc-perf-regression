use std::marker::PhantomData;

pub trait Reg<T: Send + Sync>: Send + Sync {
    fn new() -> Self;
}

macro_rules! map {
    ($ident:ident) => {
        pub struct $ident<T: Send + Sync>(PhantomData<T>);

        impl<T: Send + Sync> Reg<T> for $ident<T> {
            fn new() -> Self {
                $ident(PhantomData)
            }
        }
    };
}

map!(Foo1);
map!(Foo2);
map!(Foo3);
map!(Foo4);
map!(Foo5);
map!(Foo6);
map!(Foo7);
map!(Foo8);
map!(Foo9);
map!(Foo10);
map!(Foo11);
map!(Foo12);
map!(Foo13);
map!(Foo14);
map!(Foo15);
map!(Foo16);
map!(Foo17);
map!(Foo18);
map!(Foo19);
map!(Foo20);
map!(Foo21);
map!(Foo22);
map!(Foo23);
map!(Foo24);
map!(Foo25);
map!(Foo26);
map!(Foo27);
map!(Foo28);
map!(Foo29);
map!(Foo30);
map!(Foo31);
map!(Foo32);
map!(Foo33);
map!(Foo34);
map!(Foo35);
map!(Foo36);
map!(Foo37);
map!(Foo38);
map!(Foo39);
map!(Foo40);
map!(Foo41);
map!(Foo42);
map!(Foo43);
map!(Foo44);
map!(Foo45);
map!(Foo46);
map!(Foo47);
map!(Foo48);
map!(Foo49);
map!(Foo50);
map!(Foo51);
map!(Foo52);
map!(Foo53);
map!(Foo54);
map!(Foo55);
map!(Foo56);
map!(Foo57);
map!(Foo58);
map!(Foo59);
map!(Foo60);
map!(Foo61);
map!(Foo62);
map!(Foo63);
map!(Foo64);
map!(Foo65);
map!(Foo66);
map!(Foo67);
map!(Foo68);
map!(Foo69);
map!(Foo70);
map!(Foo71);
map!(Foo72);
map!(Foo73);
map!(Foo74);
map!(Foo75);
map!(Foo76);
map!(Foo77);
map!(Foo78);
map!(Foo79);
map!(Foo80);
map!(Foo81);
map!(Foo82);
map!(Foo83);
map!(Foo84);
map!(Foo85);
map!(Foo86);
map!(Foo87);
map!(Foo88);
map!(Foo89);
map!(Foo90);
map!(Foo91);
map!(Foo92);
map!(Foo93);
map!(Foo94);
map!(Foo95);
map!(Foo96);
map!(Foo97);
map!(Foo98);
map!(Foo99);
map!(Foo100);
map!(Foo101);
map!(Foo102);
map!(Foo103);
map!(Foo104);
map!(Foo105);
map!(Foo106);
map!(Foo107);
map!(Foo108);
map!(Foo109);
map!(Foo110);
map!(Foo111);
map!(Foo112);
map!(Foo113);
map!(Foo114);
map!(Foo115);
map!(Foo116);
map!(Foo117);
map!(Foo118);
map!(Foo119);
map!(Foo120);
map!(Foo121);
map!(Foo122);
map!(Foo123);
map!(Foo124);
map!(Foo125);
map!(Foo126);
map!(Foo127);
map!(Foo128);
map!(Foo129);
map!(Foo130);
map!(Foo131);
map!(Foo132);
map!(Foo133);
map!(Foo134);
map!(Foo135);
map!(Foo136);
map!(Foo137);
map!(Foo138);
map!(Foo139);
map!(Foo140);
map!(Foo141);
map!(Foo142);
map!(Foo143);
map!(Foo144);
map!(Foo145);
map!(Foo146);
map!(Foo147);
map!(Foo148);
map!(Foo149);
map!(Foo150);
map!(Foo151);
map!(Foo152);
map!(Foo153);
map!(Foo154);
map!(Foo155);
map!(Foo156);
map!(Foo157);
map!(Foo158);
map!(Foo159);
map!(Foo160);
map!(Foo161);
map!(Foo162);
map!(Foo163);
map!(Foo164);
map!(Foo165);
map!(Foo166);
map!(Foo167);
map!(Foo168);
map!(Foo169);
map!(Foo170);
map!(Foo171);
map!(Foo172);
map!(Foo173);
map!(Foo174);
map!(Foo175);
map!(Foo176);
map!(Foo177);
map!(Foo178);
map!(Foo179);
map!(Foo180);
map!(Foo181);
map!(Foo182);
map!(Foo183);
map!(Foo184);
map!(Foo185);
map!(Foo186);
map!(Foo187);
map!(Foo188);
map!(Foo189);
map!(Foo190);
map!(Foo191);
map!(Foo192);
map!(Foo193);
map!(Foo194);
map!(Foo195);
map!(Foo196);
map!(Foo197);
map!(Foo198);
map!(Foo199);
map!(Foo200);
map!(Foo201);
map!(Foo202);
map!(Foo203);
map!(Foo204);
map!(Foo205);
map!(Foo206);
map!(Foo207);
map!(Foo208);
map!(Foo209);
map!(Foo210);
map!(Foo211);
map!(Foo212);
map!(Foo213);
map!(Foo214);
map!(Foo215);
map!(Foo216);
map!(Foo217);
map!(Foo218);
map!(Foo219);
map!(Foo220);
map!(Foo221);
map!(Foo222);
map!(Foo223);
map!(Foo224);
map!(Foo225);
map!(Foo226);
map!(Foo227);
map!(Foo228);
map!(Foo229);
map!(Foo230);
map!(Foo231);
map!(Foo232);
map!(Foo233);
map!(Foo234);
map!(Foo235);
map!(Foo236);
map!(Foo237);
map!(Foo238);
map!(Foo239);
map!(Foo240);
map!(Foo241);
map!(Foo242);
map!(Foo243);
map!(Foo244);
map!(Foo245);
map!(Foo246);
map!(Foo247);
map!(Foo248);
map!(Foo249);
map!(Foo250);
map!(Foo251);
map!(Foo252);
map!(Foo253);
map!(Foo254);
map!(Foo255);
map!(Foo256);
map!(Foo257);
map!(Foo258);
map!(Foo259);
map!(Foo260);
map!(Foo261);
map!(Foo262);
map!(Foo263);
map!(Foo264);
map!(Foo265);
map!(Foo266);
map!(Foo267);
map!(Foo268);
map!(Foo269);
map!(Foo270);
map!(Foo271);
map!(Foo272);
map!(Foo273);
map!(Foo274);
map!(Foo275);
map!(Foo276);
map!(Foo277);
map!(Foo278);
map!(Foo279);
map!(Foo280);
map!(Foo281);
map!(Foo282);
map!(Foo283);
map!(Foo284);
map!(Foo285);
map!(Foo286);
map!(Foo287);
map!(Foo288);
map!(Foo289);
map!(Foo290);
map!(Foo291);
map!(Foo292);
map!(Foo293);
map!(Foo294);
map!(Foo295);
map!(Foo296);
map!(Foo297);
map!(Foo298);
map!(Foo299);
map!(Foo300);
map!(Foo301);
map!(Foo302);
map!(Foo303);
map!(Foo304);
map!(Foo305);
map!(Foo306);
map!(Foo307);
map!(Foo308);
map!(Foo309);
map!(Foo310);
map!(Foo311);
map!(Foo312);
map!(Foo313);
map!(Foo314);
map!(Foo315);
map!(Foo316);
map!(Foo317);
map!(Foo318);
map!(Foo319);
map!(Foo320);
map!(Foo321);
map!(Foo322);
map!(Foo323);
map!(Foo324);
map!(Foo325);
map!(Foo326);
map!(Foo327);
map!(Foo328);
map!(Foo329);
map!(Foo330);
map!(Foo331);
map!(Foo332);
map!(Foo333);
map!(Foo334);
map!(Foo335);
map!(Foo336);
map!(Foo337);
map!(Foo338);
map!(Foo339);
map!(Foo340);
map!(Foo341);
map!(Foo342);
map!(Foo343);
map!(Foo344);
map!(Foo345);
map!(Foo346);
map!(Foo347);
map!(Foo348);
map!(Foo349);
map!(Foo350);
map!(Foo351);
map!(Foo352);
map!(Foo353);
map!(Foo354);
map!(Foo355);
map!(Foo356);
map!(Foo357);
map!(Foo358);
map!(Foo359);
map!(Foo360);
map!(Foo361);
map!(Foo362);
map!(Foo363);
map!(Foo364);
map!(Foo365);
map!(Foo366);
map!(Foo367);
map!(Foo368);
map!(Foo369);
map!(Foo370);
map!(Foo371);
map!(Foo372);
map!(Foo373);
map!(Foo374);
map!(Foo375);
map!(Foo376);
map!(Foo377);
map!(Foo378);
map!(Foo379);
map!(Foo380);
map!(Foo381);
map!(Foo382);
map!(Foo383);
map!(Foo384);
map!(Foo385);
map!(Foo386);
map!(Foo387);
map!(Foo388);
map!(Foo389);
map!(Foo390);
map!(Foo391);
map!(Foo392);
map!(Foo393);
map!(Foo394);
map!(Foo395);
map!(Foo396);
map!(Foo397);
map!(Foo398);
map!(Foo399);
map!(Foo400);
map!(Foo401);
map!(Foo402);
map!(Foo403);
map!(Foo404);
map!(Foo405);
map!(Foo406);
map!(Foo407);
map!(Foo408);
map!(Foo409);
map!(Foo410);
map!(Foo411);
map!(Foo412);
map!(Foo413);
map!(Foo414);
map!(Foo415);
map!(Foo416);
map!(Foo417);
map!(Foo418);
map!(Foo419);
map!(Foo420);
map!(Foo421);
map!(Foo422);
map!(Foo423);
map!(Foo424);
map!(Foo425);
map!(Foo426);
map!(Foo427);
map!(Foo428);
map!(Foo429);
map!(Foo430);
map!(Foo431);
map!(Foo432);
map!(Foo433);
map!(Foo434);
map!(Foo435);
map!(Foo436);
map!(Foo437);
map!(Foo438);
map!(Foo439);
map!(Foo440);
map!(Foo441);
map!(Foo442);
map!(Foo443);
map!(Foo444);
map!(Foo445);
map!(Foo446);
map!(Foo447);
map!(Foo448);
map!(Foo449);
map!(Foo450);
map!(Foo451);
map!(Foo452);
map!(Foo453);
map!(Foo454);
map!(Foo455);
map!(Foo456);
map!(Foo457);
map!(Foo458);
map!(Foo459);
map!(Foo460);
map!(Foo461);
map!(Foo462);
map!(Foo463);
map!(Foo464);
map!(Foo465);
map!(Foo466);
map!(Foo467);
map!(Foo468);
map!(Foo469);
map!(Foo470);
map!(Foo471);
map!(Foo472);
map!(Foo473);
map!(Foo474);
map!(Foo475);
map!(Foo476);
map!(Foo477);
map!(Foo478);
map!(Foo479);
map!(Foo480);
map!(Foo481);
map!(Foo482);
map!(Foo483);
map!(Foo484);
map!(Foo485);
map!(Foo486);
map!(Foo487);
map!(Foo488);
map!(Foo489);
map!(Foo490);
map!(Foo491);
map!(Foo492);
map!(Foo493);
map!(Foo494);
map!(Foo495);
map!(Foo496);
map!(Foo497);
map!(Foo498);
map!(Foo499);
map!(Foo500);
map!(Foo501);
map!(Foo502);
map!(Foo503);
map!(Foo504);
map!(Foo505);
map!(Foo506);
map!(Foo507);
map!(Foo508);
map!(Foo509);
map!(Foo510);
map!(Foo511);
map!(Foo512);
map!(Foo513);
map!(Foo514);
map!(Foo515);
map!(Foo516);
map!(Foo517);
map!(Foo518);
map!(Foo519);
map!(Foo520);
map!(Foo521);
map!(Foo522);
map!(Foo523);
map!(Foo524);
map!(Foo525);
map!(Foo526);
map!(Foo527);
map!(Foo528);
map!(Foo529);
map!(Foo530);
map!(Foo531);
map!(Foo532);
map!(Foo533);
map!(Foo534);
map!(Foo535);
map!(Foo536);
map!(Foo537);
map!(Foo538);
map!(Foo539);
map!(Foo540);
map!(Foo541);
map!(Foo542);
map!(Foo543);
map!(Foo544);
map!(Foo545);
map!(Foo546);
map!(Foo547);
map!(Foo548);
map!(Foo549);
map!(Foo550);
map!(Foo551);
map!(Foo552);
map!(Foo553);
map!(Foo554);
map!(Foo555);
map!(Foo556);
map!(Foo557);
map!(Foo558);
map!(Foo559);
map!(Foo560);
map!(Foo561);
map!(Foo562);
map!(Foo563);
map!(Foo564);
map!(Foo565);
map!(Foo566);
map!(Foo567);
map!(Foo568);
map!(Foo569);
map!(Foo570);
map!(Foo571);
map!(Foo572);
map!(Foo573);
map!(Foo574);
map!(Foo575);
map!(Foo576);
map!(Foo577);
map!(Foo578);
map!(Foo579);
map!(Foo580);
map!(Foo581);
map!(Foo582);
map!(Foo583);
map!(Foo584);
map!(Foo585);
map!(Foo586);
map!(Foo587);
map!(Foo588);
map!(Foo589);
map!(Foo590);
map!(Foo591);
map!(Foo592);
map!(Foo593);
map!(Foo594);
map!(Foo595);
map!(Foo596);
map!(Foo597);
map!(Foo598);
map!(Foo599);
map!(Foo600);
map!(Foo601);
map!(Foo602);
map!(Foo603);
map!(Foo604);
map!(Foo605);
map!(Foo606);
map!(Foo607);
map!(Foo608);
map!(Foo609);
map!(Foo610);
map!(Foo611);
map!(Foo612);
map!(Foo613);
map!(Foo614);
map!(Foo615);
map!(Foo616);
map!(Foo617);
map!(Foo618);
map!(Foo619);
map!(Foo620);
map!(Foo621);
map!(Foo622);
map!(Foo623);
map!(Foo624);
map!(Foo625);
map!(Foo626);
map!(Foo627);
map!(Foo628);
map!(Foo629);
map!(Foo630);
map!(Foo631);
map!(Foo632);
map!(Foo633);
map!(Foo634);
map!(Foo635);
map!(Foo636);
map!(Foo637);
map!(Foo638);
map!(Foo639);
map!(Foo640);
map!(Foo641);
map!(Foo642);
map!(Foo643);
map!(Foo644);
map!(Foo645);
map!(Foo646);
map!(Foo647);
map!(Foo648);
map!(Foo649);
map!(Foo650);
map!(Foo651);
map!(Foo652);
map!(Foo653);
map!(Foo654);
map!(Foo655);
map!(Foo656);
map!(Foo657);
map!(Foo658);
map!(Foo659);
map!(Foo660);
map!(Foo661);
map!(Foo662);
map!(Foo663);
map!(Foo664);
map!(Foo665);
map!(Foo666);
map!(Foo667);
map!(Foo668);
map!(Foo669);
map!(Foo670);
map!(Foo671);
map!(Foo672);
map!(Foo673);
map!(Foo674);
map!(Foo675);
map!(Foo676);
map!(Foo677);
map!(Foo678);
map!(Foo679);
map!(Foo680);
map!(Foo681);
map!(Foo682);
map!(Foo683);
map!(Foo684);
map!(Foo685);
map!(Foo686);
map!(Foo687);
map!(Foo688);
map!(Foo689);
map!(Foo690);
map!(Foo691);
map!(Foo692);
map!(Foo693);
map!(Foo694);
map!(Foo695);
map!(Foo696);
map!(Foo697);
map!(Foo698);
map!(Foo699);
map!(Foo700);
map!(Foo701);
map!(Foo702);
map!(Foo703);
map!(Foo704);
map!(Foo705);
map!(Foo706);
map!(Foo707);
map!(Foo708);
map!(Foo709);
map!(Foo710);
map!(Foo711);
map!(Foo712);
map!(Foo713);
map!(Foo714);
map!(Foo715);
map!(Foo716);
map!(Foo717);
map!(Foo718);
map!(Foo719);
map!(Foo720);
map!(Foo721);
map!(Foo722);
map!(Foo723);
map!(Foo724);
map!(Foo725);
map!(Foo726);
map!(Foo727);
map!(Foo728);
map!(Foo729);
map!(Foo730);
map!(Foo731);
map!(Foo732);
map!(Foo733);
map!(Foo734);
map!(Foo735);
map!(Foo736);
map!(Foo737);
map!(Foo738);
map!(Foo739);
map!(Foo740);
map!(Foo741);
map!(Foo742);
map!(Foo743);
map!(Foo744);
map!(Foo745);
map!(Foo746);
map!(Foo747);
map!(Foo748);
map!(Foo749);
map!(Foo750);
map!(Foo751);
map!(Foo752);
map!(Foo753);
map!(Foo754);
map!(Foo755);
map!(Foo756);
map!(Foo757);
map!(Foo758);
map!(Foo759);
map!(Foo760);
map!(Foo761);
map!(Foo762);
map!(Foo763);
map!(Foo764);
map!(Foo765);
map!(Foo766);
map!(Foo767);
map!(Foo768);
map!(Foo769);
map!(Foo770);
map!(Foo771);
map!(Foo772);
map!(Foo773);
map!(Foo774);
map!(Foo775);
map!(Foo776);
map!(Foo777);
map!(Foo778);
map!(Foo779);
map!(Foo780);
map!(Foo781);
map!(Foo782);
map!(Foo783);
map!(Foo784);
map!(Foo785);
map!(Foo786);
map!(Foo787);
map!(Foo788);
map!(Foo789);
map!(Foo790);
map!(Foo791);
map!(Foo792);
map!(Foo793);
map!(Foo794);
map!(Foo795);
map!(Foo796);
map!(Foo797);
map!(Foo798);
map!(Foo799);
map!(Foo800);
map!(Foo801);
map!(Foo802);
map!(Foo803);
map!(Foo804);
map!(Foo805);
map!(Foo806);
map!(Foo807);
map!(Foo808);
map!(Foo809);
map!(Foo810);
map!(Foo811);
map!(Foo812);
map!(Foo813);
map!(Foo814);
map!(Foo815);
map!(Foo816);
map!(Foo817);
map!(Foo818);
map!(Foo819);
map!(Foo820);
map!(Foo821);
map!(Foo822);
map!(Foo823);
map!(Foo824);
map!(Foo825);
map!(Foo826);
map!(Foo827);
map!(Foo828);
map!(Foo829);
map!(Foo830);
map!(Foo831);
map!(Foo832);
map!(Foo833);
map!(Foo834);
map!(Foo835);
map!(Foo836);
map!(Foo837);
map!(Foo838);
map!(Foo839);
map!(Foo840);
map!(Foo841);
map!(Foo842);
map!(Foo843);
map!(Foo844);
map!(Foo845);
map!(Foo846);
map!(Foo847);
map!(Foo848);
map!(Foo849);
map!(Foo850);
map!(Foo851);
map!(Foo852);
map!(Foo853);
map!(Foo854);
map!(Foo855);
map!(Foo856);
map!(Foo857);
map!(Foo858);
map!(Foo859);
map!(Foo860);
map!(Foo861);
map!(Foo862);
map!(Foo863);
map!(Foo864);
map!(Foo865);
map!(Foo866);
map!(Foo867);
map!(Foo868);
map!(Foo869);
map!(Foo870);
map!(Foo871);
map!(Foo872);
map!(Foo873);
map!(Foo874);
map!(Foo875);
map!(Foo876);
map!(Foo877);
map!(Foo878);
map!(Foo879);
map!(Foo880);
map!(Foo881);
map!(Foo882);
map!(Foo883);
map!(Foo884);
map!(Foo885);
map!(Foo886);
map!(Foo887);
map!(Foo888);
map!(Foo889);
map!(Foo890);
map!(Foo891);
map!(Foo892);
map!(Foo893);
map!(Foo894);
map!(Foo895);
map!(Foo896);
map!(Foo897);
map!(Foo898);
map!(Foo899);
map!(Foo900);
map!(Foo901);
map!(Foo902);
map!(Foo903);
map!(Foo904);
map!(Foo905);
map!(Foo906);
map!(Foo907);
map!(Foo908);
map!(Foo909);
map!(Foo910);
map!(Foo911);
map!(Foo912);
map!(Foo913);
map!(Foo914);
map!(Foo915);
map!(Foo916);
map!(Foo917);
map!(Foo918);
map!(Foo919);
map!(Foo920);
map!(Foo921);
map!(Foo922);
map!(Foo923);
map!(Foo924);
map!(Foo925);
map!(Foo926);
map!(Foo927);
map!(Foo928);
map!(Foo929);
map!(Foo930);
map!(Foo931);
map!(Foo932);
map!(Foo933);
map!(Foo934);
map!(Foo935);
map!(Foo936);
map!(Foo937);
map!(Foo938);
map!(Foo939);
map!(Foo940);
map!(Foo941);
map!(Foo942);
map!(Foo943);
map!(Foo944);
map!(Foo945);
map!(Foo946);
map!(Foo947);
map!(Foo948);
map!(Foo949);
map!(Foo950);
map!(Foo951);
map!(Foo952);
map!(Foo953);
map!(Foo954);
map!(Foo955);
map!(Foo956);
map!(Foo957);
map!(Foo958);
map!(Foo959);
map!(Foo960);
map!(Foo961);
map!(Foo962);
map!(Foo963);
map!(Foo964);
map!(Foo965);
map!(Foo966);
map!(Foo967);
map!(Foo968);
map!(Foo969);
map!(Foo970);
map!(Foo971);
map!(Foo972);
map!(Foo973);
map!(Foo974);
map!(Foo975);
map!(Foo976);
map!(Foo977);
map!(Foo978);
map!(Foo979);
map!(Foo980);
map!(Foo981);
map!(Foo982);
map!(Foo983);
map!(Foo984);
map!(Foo985);
map!(Foo986);
map!(Foo987);
map!(Foo988);
map!(Foo989);
map!(Foo990);
map!(Foo991);
map!(Foo992);
map!(Foo993);
map!(Foo994);
map!(Foo995);
map!(Foo996);
map!(Foo997);
map!(Foo998);
map!(Foo999);
map!(Foo1000);
map!(Foo1001);
map!(Foo1002);
map!(Foo1003);
map!(Foo1004);
map!(Foo1005);
map!(Foo1006);
map!(Foo1007);
map!(Foo1008);
map!(Foo1009);
map!(Foo1010);
map!(Foo1011);
map!(Foo1012);
map!(Foo1013);
map!(Foo1014);
map!(Foo1015);
map!(Foo1016);
map!(Foo1017);
map!(Foo1018);
map!(Foo1019);
map!(Foo1020);
map!(Foo1021);
map!(Foo1022);
map!(Foo1023);
map!(Foo1024);
map!(Foo1025);
map!(Foo1026);
map!(Foo1027);
map!(Foo1028);
map!(Foo1029);
map!(Foo1030);
map!(Foo1031);
map!(Foo1032);
map!(Foo1033);
map!(Foo1034);
map!(Foo1035);
map!(Foo1036);
map!(Foo1037);
map!(Foo1038);
map!(Foo1039);
map!(Foo1040);
map!(Foo1041);
map!(Foo1042);
map!(Foo1043);
map!(Foo1044);
map!(Foo1045);
map!(Foo1046);
map!(Foo1047);
map!(Foo1048);
map!(Foo1049);
map!(Foo1050);
map!(Foo1051);
map!(Foo1052);
map!(Foo1053);
map!(Foo1054);
map!(Foo1055);
map!(Foo1056);
map!(Foo1057);
map!(Foo1058);
map!(Foo1059);
map!(Foo1060);
map!(Foo1061);
map!(Foo1062);
map!(Foo1063);
map!(Foo1064);
map!(Foo1065);
map!(Foo1066);
map!(Foo1067);
map!(Foo1068);
map!(Foo1069);
map!(Foo1070);
map!(Foo1071);
map!(Foo1072);
map!(Foo1073);
map!(Foo1074);
map!(Foo1075);
map!(Foo1076);
map!(Foo1077);
map!(Foo1078);
map!(Foo1079);
map!(Foo1080);
map!(Foo1081);
map!(Foo1082);
map!(Foo1083);
map!(Foo1084);
map!(Foo1085);
map!(Foo1086);
map!(Foo1087);
map!(Foo1088);
map!(Foo1089);
map!(Foo1090);
map!(Foo1091);
map!(Foo1092);
map!(Foo1093);
map!(Foo1094);
map!(Foo1095);
map!(Foo1096);
map!(Foo1097);
map!(Foo1098);
map!(Foo1099);
map!(Foo1100);
map!(Foo1101);
map!(Foo1102);
map!(Foo1103);
map!(Foo1104);
map!(Foo1105);
map!(Foo1106);
map!(Foo1107);
map!(Foo1108);
map!(Foo1109);
map!(Foo1110);
map!(Foo1111);
map!(Foo1112);
map!(Foo1113);
map!(Foo1114);
map!(Foo1115);
map!(Foo1116);
map!(Foo1117);
map!(Foo1118);
map!(Foo1119);
map!(Foo1120);
map!(Foo1121);
map!(Foo1122);
map!(Foo1123);
map!(Foo1124);
map!(Foo1125);
map!(Foo1126);
map!(Foo1127);
map!(Foo1128);
map!(Foo1129);
map!(Foo1130);
map!(Foo1131);
map!(Foo1132);
map!(Foo1133);
map!(Foo1134);
map!(Foo1135);
map!(Foo1136);
map!(Foo1137);
map!(Foo1138);
map!(Foo1139);
map!(Foo1140);
map!(Foo1141);
map!(Foo1142);
map!(Foo1143);
map!(Foo1144);
map!(Foo1145);
map!(Foo1146);
map!(Foo1147);
map!(Foo1148);
map!(Foo1149);
map!(Foo1150);
map!(Foo1151);
map!(Foo1152);
map!(Foo1153);
map!(Foo1154);
map!(Foo1155);
map!(Foo1156);
map!(Foo1157);
map!(Foo1158);
map!(Foo1159);
map!(Foo1160);
map!(Foo1161);
map!(Foo1162);
map!(Foo1163);
map!(Foo1164);
map!(Foo1165);
map!(Foo1166);
map!(Foo1167);
map!(Foo1168);
map!(Foo1169);
map!(Foo1170);
map!(Foo1171);
map!(Foo1172);
map!(Foo1173);
map!(Foo1174);
map!(Foo1175);
map!(Foo1176);
map!(Foo1177);
map!(Foo1178);
map!(Foo1179);
map!(Foo1180);
map!(Foo1181);
map!(Foo1182);
map!(Foo1183);
map!(Foo1184);
map!(Foo1185);
map!(Foo1186);
map!(Foo1187);
map!(Foo1188);
map!(Foo1189);
map!(Foo1190);
map!(Foo1191);
map!(Foo1192);
map!(Foo1193);
map!(Foo1194);
map!(Foo1195);
map!(Foo1196);
map!(Foo1197);
map!(Foo1198);
map!(Foo1199);
map!(Foo1200);
map!(Foo1201);
map!(Foo1202);
map!(Foo1203);
map!(Foo1204);
map!(Foo1205);
map!(Foo1206);
map!(Foo1207);
map!(Foo1208);
map!(Foo1209);
map!(Foo1210);
map!(Foo1211);
map!(Foo1212);
map!(Foo1213);
map!(Foo1214);
map!(Foo1215);
map!(Foo1216);
map!(Foo1217);
map!(Foo1218);
map!(Foo1219);
map!(Foo1220);
map!(Foo1221);
map!(Foo1222);
map!(Foo1223);
map!(Foo1224);
map!(Foo1225);
map!(Foo1226);
map!(Foo1227);
map!(Foo1228);
map!(Foo1229);
map!(Foo1230);
map!(Foo1231);
map!(Foo1232);
map!(Foo1233);
map!(Foo1234);
map!(Foo1235);
map!(Foo1236);
map!(Foo1237);
map!(Foo1238);
map!(Foo1239);
map!(Foo1240);
map!(Foo1241);
map!(Foo1242);
map!(Foo1243);
map!(Foo1244);
map!(Foo1245);
map!(Foo1246);
map!(Foo1247);
map!(Foo1248);
map!(Foo1249);
map!(Foo1250);
map!(Foo1251);
map!(Foo1252);
map!(Foo1253);
map!(Foo1254);
map!(Foo1255);
map!(Foo1256);
map!(Foo1257);
map!(Foo1258);
map!(Foo1259);
map!(Foo1260);
map!(Foo1261);
map!(Foo1262);
map!(Foo1263);
map!(Foo1264);
map!(Foo1265);
map!(Foo1266);
map!(Foo1267);
map!(Foo1268);
map!(Foo1269);
map!(Foo1270);
map!(Foo1271);
map!(Foo1272);
map!(Foo1273);
map!(Foo1274);
map!(Foo1275);
map!(Foo1276);
map!(Foo1277);
map!(Foo1278);
map!(Foo1279);
map!(Foo1280);
map!(Foo1281);
map!(Foo1282);
map!(Foo1283);
map!(Foo1284);
map!(Foo1285);
map!(Foo1286);
map!(Foo1287);
map!(Foo1288);
map!(Foo1289);
map!(Foo1290);
map!(Foo1291);
map!(Foo1292);
map!(Foo1293);
map!(Foo1294);
map!(Foo1295);
map!(Foo1296);
map!(Foo1297);
map!(Foo1298);
map!(Foo1299);
map!(Foo1300);
map!(Foo1301);
map!(Foo1302);
map!(Foo1303);
map!(Foo1304);
map!(Foo1305);
map!(Foo1306);
map!(Foo1307);
map!(Foo1308);
map!(Foo1309);
map!(Foo1310);
map!(Foo1311);
map!(Foo1312);
map!(Foo1313);
map!(Foo1314);
map!(Foo1315);
map!(Foo1316);
map!(Foo1317);
map!(Foo1318);
map!(Foo1319);
map!(Foo1320);
map!(Foo1321);
map!(Foo1322);
map!(Foo1323);
map!(Foo1324);
map!(Foo1325);
map!(Foo1326);
map!(Foo1327);
map!(Foo1328);
map!(Foo1329);
map!(Foo1330);
map!(Foo1331);
map!(Foo1332);
map!(Foo1333);
map!(Foo1334);
map!(Foo1335);
map!(Foo1336);
map!(Foo1337);
map!(Foo1338);
map!(Foo1339);
map!(Foo1340);
map!(Foo1341);
map!(Foo1342);
map!(Foo1343);
map!(Foo1344);
map!(Foo1345);
map!(Foo1346);
map!(Foo1347);
map!(Foo1348);
map!(Foo1349);
map!(Foo1350);
map!(Foo1351);
map!(Foo1352);
map!(Foo1353);
map!(Foo1354);
map!(Foo1355);
map!(Foo1356);
map!(Foo1357);
map!(Foo1358);
map!(Foo1359);
map!(Foo1360);
map!(Foo1361);
map!(Foo1362);
map!(Foo1363);
map!(Foo1364);
map!(Foo1365);
map!(Foo1366);
map!(Foo1367);
map!(Foo1368);
map!(Foo1369);
map!(Foo1370);
map!(Foo1371);
map!(Foo1372);
map!(Foo1373);
map!(Foo1374);
map!(Foo1375);
map!(Foo1376);
map!(Foo1377);
map!(Foo1378);
map!(Foo1379);
map!(Foo1380);
map!(Foo1381);
map!(Foo1382);
map!(Foo1383);
map!(Foo1384);
map!(Foo1385);
map!(Foo1386);
map!(Foo1387);
map!(Foo1388);
map!(Foo1389);
map!(Foo1390);
map!(Foo1391);
map!(Foo1392);
map!(Foo1393);
map!(Foo1394);
map!(Foo1395);
map!(Foo1396);
map!(Foo1397);
map!(Foo1398);
map!(Foo1399);
map!(Foo1400);
map!(Foo1401);
map!(Foo1402);
map!(Foo1403);
map!(Foo1404);
map!(Foo1405);
map!(Foo1406);
map!(Foo1407);
map!(Foo1408);
map!(Foo1409);
map!(Foo1410);
map!(Foo1411);
map!(Foo1412);
map!(Foo1413);
map!(Foo1414);
map!(Foo1415);
map!(Foo1416);
map!(Foo1417);
map!(Foo1418);
map!(Foo1419);
map!(Foo1420);
map!(Foo1421);
map!(Foo1422);
map!(Foo1423);
map!(Foo1424);
map!(Foo1425);
map!(Foo1426);
map!(Foo1427);
map!(Foo1428);
map!(Foo1429);
map!(Foo1430);
map!(Foo1431);
map!(Foo1432);
map!(Foo1433);
map!(Foo1434);
map!(Foo1435);
map!(Foo1436);
map!(Foo1437);
map!(Foo1438);
map!(Foo1439);
map!(Foo1440);
map!(Foo1441);
map!(Foo1442);
map!(Foo1443);
map!(Foo1444);
map!(Foo1445);
map!(Foo1446);
map!(Foo1447);
map!(Foo1448);
map!(Foo1449);
map!(Foo1450);
map!(Foo1451);
map!(Foo1452);
map!(Foo1453);
map!(Foo1454);
map!(Foo1455);
map!(Foo1456);
map!(Foo1457);
map!(Foo1458);
map!(Foo1459);
map!(Foo1460);
map!(Foo1461);
map!(Foo1462);
map!(Foo1463);
map!(Foo1464);
map!(Foo1465);
map!(Foo1466);
map!(Foo1467);
map!(Foo1468);
map!(Foo1469);
map!(Foo1470);
map!(Foo1471);
map!(Foo1472);
map!(Foo1473);
map!(Foo1474);
map!(Foo1475);
map!(Foo1476);
map!(Foo1477);
map!(Foo1478);
map!(Foo1479);
map!(Foo1480);
map!(Foo1481);
map!(Foo1482);
map!(Foo1483);
map!(Foo1484);
map!(Foo1485);
map!(Foo1486);
map!(Foo1487);
map!(Foo1488);
map!(Foo1489);
map!(Foo1490);
map!(Foo1491);
map!(Foo1492);
map!(Foo1493);
map!(Foo1494);
map!(Foo1495);
map!(Foo1496);
map!(Foo1497);
map!(Foo1498);
map!(Foo1499);
map!(Foo1500);

pub struct Tokens {
    pub foo1: Foo1<()>,
    pub foo2: Foo2<()>,
    pub foo3: Foo3<()>,
    pub foo4: Foo4<()>,
    pub foo5: Foo5<()>,
    pub foo6: Foo6<()>,
    pub foo7: Foo7<()>,
    pub foo8: Foo8<()>,
    pub foo9: Foo9<()>,
    pub foo10: Foo10<()>,
    pub foo11: Foo11<()>,
    pub foo12: Foo12<()>,
    pub foo13: Foo13<()>,
    pub foo14: Foo14<()>,
    pub foo15: Foo15<()>,
    pub foo16: Foo16<()>,
    pub foo17: Foo17<()>,
    pub foo18: Foo18<()>,
    pub foo19: Foo19<()>,
    pub foo20: Foo20<()>,
    pub foo21: Foo21<()>,
    pub foo22: Foo22<()>,
    pub foo23: Foo23<()>,
    pub foo24: Foo24<()>,
    pub foo25: Foo25<()>,
    pub foo26: Foo26<()>,
    pub foo27: Foo27<()>,
    pub foo28: Foo28<()>,
    pub foo29: Foo29<()>,
    pub foo30: Foo30<()>,
    pub foo31: Foo31<()>,
    pub foo32: Foo32<()>,
    pub foo33: Foo33<()>,
    pub foo34: Foo34<()>,
    pub foo35: Foo35<()>,
    pub foo36: Foo36<()>,
    pub foo37: Foo37<()>,
    pub foo38: Foo38<()>,
    pub foo39: Foo39<()>,
    pub foo40: Foo40<()>,
    pub foo41: Foo41<()>,
    pub foo42: Foo42<()>,
    pub foo43: Foo43<()>,
    pub foo44: Foo44<()>,
    pub foo45: Foo45<()>,
    pub foo46: Foo46<()>,
    pub foo47: Foo47<()>,
    pub foo48: Foo48<()>,
    pub foo49: Foo49<()>,
    pub foo50: Foo50<()>,
    pub foo51: Foo51<()>,
    pub foo52: Foo52<()>,
    pub foo53: Foo53<()>,
    pub foo54: Foo54<()>,
    pub foo55: Foo55<()>,
    pub foo56: Foo56<()>,
    pub foo57: Foo57<()>,
    pub foo58: Foo58<()>,
    pub foo59: Foo59<()>,
    pub foo60: Foo60<()>,
    pub foo61: Foo61<()>,
    pub foo62: Foo62<()>,
    pub foo63: Foo63<()>,
    pub foo64: Foo64<()>,
    pub foo65: Foo65<()>,
    pub foo66: Foo66<()>,
    pub foo67: Foo67<()>,
    pub foo68: Foo68<()>,
    pub foo69: Foo69<()>,
    pub foo70: Foo70<()>,
    pub foo71: Foo71<()>,
    pub foo72: Foo72<()>,
    pub foo73: Foo73<()>,
    pub foo74: Foo74<()>,
    pub foo75: Foo75<()>,
    pub foo76: Foo76<()>,
    pub foo77: Foo77<()>,
    pub foo78: Foo78<()>,
    pub foo79: Foo79<()>,
    pub foo80: Foo80<()>,
    pub foo81: Foo81<()>,
    pub foo82: Foo82<()>,
    pub foo83: Foo83<()>,
    pub foo84: Foo84<()>,
    pub foo85: Foo85<()>,
    pub foo86: Foo86<()>,
    pub foo87: Foo87<()>,
    pub foo88: Foo88<()>,
    pub foo89: Foo89<()>,
    pub foo90: Foo90<()>,
    pub foo91: Foo91<()>,
    pub foo92: Foo92<()>,
    pub foo93: Foo93<()>,
    pub foo94: Foo94<()>,
    pub foo95: Foo95<()>,
    pub foo96: Foo96<()>,
    pub foo97: Foo97<()>,
    pub foo98: Foo98<()>,
    pub foo99: Foo99<()>,
    pub foo100: Foo100<()>,
    pub foo101: Foo101<()>,
    pub foo102: Foo102<()>,
    pub foo103: Foo103<()>,
    pub foo104: Foo104<()>,
    pub foo105: Foo105<()>,
    pub foo106: Foo106<()>,
    pub foo107: Foo107<()>,
    pub foo108: Foo108<()>,
    pub foo109: Foo109<()>,
    pub foo110: Foo110<()>,
    pub foo111: Foo111<()>,
    pub foo112: Foo112<()>,
    pub foo113: Foo113<()>,
    pub foo114: Foo114<()>,
    pub foo115: Foo115<()>,
    pub foo116: Foo116<()>,
    pub foo117: Foo117<()>,
    pub foo118: Foo118<()>,
    pub foo119: Foo119<()>,
    pub foo120: Foo120<()>,
    pub foo121: Foo121<()>,
    pub foo122: Foo122<()>,
    pub foo123: Foo123<()>,
    pub foo124: Foo124<()>,
    pub foo125: Foo125<()>,
    pub foo126: Foo126<()>,
    pub foo127: Foo127<()>,
    pub foo128: Foo128<()>,
    pub foo129: Foo129<()>,
    pub foo130: Foo130<()>,
    pub foo131: Foo131<()>,
    pub foo132: Foo132<()>,
    pub foo133: Foo133<()>,
    pub foo134: Foo134<()>,
    pub foo135: Foo135<()>,
    pub foo136: Foo136<()>,
    pub foo137: Foo137<()>,
    pub foo138: Foo138<()>,
    pub foo139: Foo139<()>,
    pub foo140: Foo140<()>,
    pub foo141: Foo141<()>,
    pub foo142: Foo142<()>,
    pub foo143: Foo143<()>,
    pub foo144: Foo144<()>,
    pub foo145: Foo145<()>,
    pub foo146: Foo146<()>,
    pub foo147: Foo147<()>,
    pub foo148: Foo148<()>,
    pub foo149: Foo149<()>,
    pub foo150: Foo150<()>,
    pub foo151: Foo151<()>,
    pub foo152: Foo152<()>,
    pub foo153: Foo153<()>,
    pub foo154: Foo154<()>,
    pub foo155: Foo155<()>,
    pub foo156: Foo156<()>,
    pub foo157: Foo157<()>,
    pub foo158: Foo158<()>,
    pub foo159: Foo159<()>,
    pub foo160: Foo160<()>,
    pub foo161: Foo161<()>,
    pub foo162: Foo162<()>,
    pub foo163: Foo163<()>,
    pub foo164: Foo164<()>,
    pub foo165: Foo165<()>,
    pub foo166: Foo166<()>,
    pub foo167: Foo167<()>,
    pub foo168: Foo168<()>,
    pub foo169: Foo169<()>,
    pub foo170: Foo170<()>,
    pub foo171: Foo171<()>,
    pub foo172: Foo172<()>,
    pub foo173: Foo173<()>,
    pub foo174: Foo174<()>,
    pub foo175: Foo175<()>,
    pub foo176: Foo176<()>,
    pub foo177: Foo177<()>,
    pub foo178: Foo178<()>,
    pub foo179: Foo179<()>,
    pub foo180: Foo180<()>,
    pub foo181: Foo181<()>,
    pub foo182: Foo182<()>,
    pub foo183: Foo183<()>,
    pub foo184: Foo184<()>,
    pub foo185: Foo185<()>,
    pub foo186: Foo186<()>,
    pub foo187: Foo187<()>,
    pub foo188: Foo188<()>,
    pub foo189: Foo189<()>,
    pub foo190: Foo190<()>,
    pub foo191: Foo191<()>,
    pub foo192: Foo192<()>,
    pub foo193: Foo193<()>,
    pub foo194: Foo194<()>,
    pub foo195: Foo195<()>,
    pub foo196: Foo196<()>,
    pub foo197: Foo197<()>,
    pub foo198: Foo198<()>,
    pub foo199: Foo199<()>,
    pub foo200: Foo200<()>,
    pub foo201: Foo201<()>,
    pub foo202: Foo202<()>,
    pub foo203: Foo203<()>,
    pub foo204: Foo204<()>,
    pub foo205: Foo205<()>,
    pub foo206: Foo206<()>,
    pub foo207: Foo207<()>,
    pub foo208: Foo208<()>,
    pub foo209: Foo209<()>,
    pub foo210: Foo210<()>,
    pub foo211: Foo211<()>,
    pub foo212: Foo212<()>,
    pub foo213: Foo213<()>,
    pub foo214: Foo214<()>,
    pub foo215: Foo215<()>,
    pub foo216: Foo216<()>,
    pub foo217: Foo217<()>,
    pub foo218: Foo218<()>,
    pub foo219: Foo219<()>,
    pub foo220: Foo220<()>,
    pub foo221: Foo221<()>,
    pub foo222: Foo222<()>,
    pub foo223: Foo223<()>,
    pub foo224: Foo224<()>,
    pub foo225: Foo225<()>,
    pub foo226: Foo226<()>,
    pub foo227: Foo227<()>,
    pub foo228: Foo228<()>,
    pub foo229: Foo229<()>,
    pub foo230: Foo230<()>,
    pub foo231: Foo231<()>,
    pub foo232: Foo232<()>,
    pub foo233: Foo233<()>,
    pub foo234: Foo234<()>,
    pub foo235: Foo235<()>,
    pub foo236: Foo236<()>,
    pub foo237: Foo237<()>,
    pub foo238: Foo238<()>,
    pub foo239: Foo239<()>,
    pub foo240: Foo240<()>,
    pub foo241: Foo241<()>,
    pub foo242: Foo242<()>,
    pub foo243: Foo243<()>,
    pub foo244: Foo244<()>,
    pub foo245: Foo245<()>,
    pub foo246: Foo246<()>,
    pub foo247: Foo247<()>,
    pub foo248: Foo248<()>,
    pub foo249: Foo249<()>,
    pub foo250: Foo250<()>,
    pub foo251: Foo251<()>,
    pub foo252: Foo252<()>,
    pub foo253: Foo253<()>,
    pub foo254: Foo254<()>,
    pub foo255: Foo255<()>,
    pub foo256: Foo256<()>,
    pub foo257: Foo257<()>,
    pub foo258: Foo258<()>,
    pub foo259: Foo259<()>,
    pub foo260: Foo260<()>,
    pub foo261: Foo261<()>,
    pub foo262: Foo262<()>,
    pub foo263: Foo263<()>,
    pub foo264: Foo264<()>,
    pub foo265: Foo265<()>,
    pub foo266: Foo266<()>,
    pub foo267: Foo267<()>,
    pub foo268: Foo268<()>,
    pub foo269: Foo269<()>,
    pub foo270: Foo270<()>,
    pub foo271: Foo271<()>,
    pub foo272: Foo272<()>,
    pub foo273: Foo273<()>,
    pub foo274: Foo274<()>,
    pub foo275: Foo275<()>,
    pub foo276: Foo276<()>,
    pub foo277: Foo277<()>,
    pub foo278: Foo278<()>,
    pub foo279: Foo279<()>,
    pub foo280: Foo280<()>,
    pub foo281: Foo281<()>,
    pub foo282: Foo282<()>,
    pub foo283: Foo283<()>,
    pub foo284: Foo284<()>,
    pub foo285: Foo285<()>,
    pub foo286: Foo286<()>,
    pub foo287: Foo287<()>,
    pub foo288: Foo288<()>,
    pub foo289: Foo289<()>,
    pub foo290: Foo290<()>,
    pub foo291: Foo291<()>,
    pub foo292: Foo292<()>,
    pub foo293: Foo293<()>,
    pub foo294: Foo294<()>,
    pub foo295: Foo295<()>,
    pub foo296: Foo296<()>,
    pub foo297: Foo297<()>,
    pub foo298: Foo298<()>,
    pub foo299: Foo299<()>,
    pub foo300: Foo300<()>,
    pub foo301: Foo301<()>,
    pub foo302: Foo302<()>,
    pub foo303: Foo303<()>,
    pub foo304: Foo304<()>,
    pub foo305: Foo305<()>,
    pub foo306: Foo306<()>,
    pub foo307: Foo307<()>,
    pub foo308: Foo308<()>,
    pub foo309: Foo309<()>,
    pub foo310: Foo310<()>,
    pub foo311: Foo311<()>,
    pub foo312: Foo312<()>,
    pub foo313: Foo313<()>,
    pub foo314: Foo314<()>,
    pub foo315: Foo315<()>,
    pub foo316: Foo316<()>,
    pub foo317: Foo317<()>,
    pub foo318: Foo318<()>,
    pub foo319: Foo319<()>,
    pub foo320: Foo320<()>,
    pub foo321: Foo321<()>,
    pub foo322: Foo322<()>,
    pub foo323: Foo323<()>,
    pub foo324: Foo324<()>,
    pub foo325: Foo325<()>,
    pub foo326: Foo326<()>,
    pub foo327: Foo327<()>,
    pub foo328: Foo328<()>,
    pub foo329: Foo329<()>,
    pub foo330: Foo330<()>,
    pub foo331: Foo331<()>,
    pub foo332: Foo332<()>,
    pub foo333: Foo333<()>,
    pub foo334: Foo334<()>,
    pub foo335: Foo335<()>,
    pub foo336: Foo336<()>,
    pub foo337: Foo337<()>,
    pub foo338: Foo338<()>,
    pub foo339: Foo339<()>,
    pub foo340: Foo340<()>,
    pub foo341: Foo341<()>,
    pub foo342: Foo342<()>,
    pub foo343: Foo343<()>,
    pub foo344: Foo344<()>,
    pub foo345: Foo345<()>,
    pub foo346: Foo346<()>,
    pub foo347: Foo347<()>,
    pub foo348: Foo348<()>,
    pub foo349: Foo349<()>,
    pub foo350: Foo350<()>,
    pub foo351: Foo351<()>,
    pub foo352: Foo352<()>,
    pub foo353: Foo353<()>,
    pub foo354: Foo354<()>,
    pub foo355: Foo355<()>,
    pub foo356: Foo356<()>,
    pub foo357: Foo357<()>,
    pub foo358: Foo358<()>,
    pub foo359: Foo359<()>,
    pub foo360: Foo360<()>,
    pub foo361: Foo361<()>,
    pub foo362: Foo362<()>,
    pub foo363: Foo363<()>,
    pub foo364: Foo364<()>,
    pub foo365: Foo365<()>,
    pub foo366: Foo366<()>,
    pub foo367: Foo367<()>,
    pub foo368: Foo368<()>,
    pub foo369: Foo369<()>,
    pub foo370: Foo370<()>,
    pub foo371: Foo371<()>,
    pub foo372: Foo372<()>,
    pub foo373: Foo373<()>,
    pub foo374: Foo374<()>,
    pub foo375: Foo375<()>,
    pub foo376: Foo376<()>,
    pub foo377: Foo377<()>,
    pub foo378: Foo378<()>,
    pub foo379: Foo379<()>,
    pub foo380: Foo380<()>,
    pub foo381: Foo381<()>,
    pub foo382: Foo382<()>,
    pub foo383: Foo383<()>,
    pub foo384: Foo384<()>,
    pub foo385: Foo385<()>,
    pub foo386: Foo386<()>,
    pub foo387: Foo387<()>,
    pub foo388: Foo388<()>,
    pub foo389: Foo389<()>,
    pub foo390: Foo390<()>,
    pub foo391: Foo391<()>,
    pub foo392: Foo392<()>,
    pub foo393: Foo393<()>,
    pub foo394: Foo394<()>,
    pub foo395: Foo395<()>,
    pub foo396: Foo396<()>,
    pub foo397: Foo397<()>,
    pub foo398: Foo398<()>,
    pub foo399: Foo399<()>,
    pub foo400: Foo400<()>,
    pub foo401: Foo401<()>,
    pub foo402: Foo402<()>,
    pub foo403: Foo403<()>,
    pub foo404: Foo404<()>,
    pub foo405: Foo405<()>,
    pub foo406: Foo406<()>,
    pub foo407: Foo407<()>,
    pub foo408: Foo408<()>,
    pub foo409: Foo409<()>,
    pub foo410: Foo410<()>,
    pub foo411: Foo411<()>,
    pub foo412: Foo412<()>,
    pub foo413: Foo413<()>,
    pub foo414: Foo414<()>,
    pub foo415: Foo415<()>,
    pub foo416: Foo416<()>,
    pub foo417: Foo417<()>,
    pub foo418: Foo418<()>,
    pub foo419: Foo419<()>,
    pub foo420: Foo420<()>,
    pub foo421: Foo421<()>,
    pub foo422: Foo422<()>,
    pub foo423: Foo423<()>,
    pub foo424: Foo424<()>,
    pub foo425: Foo425<()>,
    pub foo426: Foo426<()>,
    pub foo427: Foo427<()>,
    pub foo428: Foo428<()>,
    pub foo429: Foo429<()>,
    pub foo430: Foo430<()>,
    pub foo431: Foo431<()>,
    pub foo432: Foo432<()>,
    pub foo433: Foo433<()>,
    pub foo434: Foo434<()>,
    pub foo435: Foo435<()>,
    pub foo436: Foo436<()>,
    pub foo437: Foo437<()>,
    pub foo438: Foo438<()>,
    pub foo439: Foo439<()>,
    pub foo440: Foo440<()>,
    pub foo441: Foo441<()>,
    pub foo442: Foo442<()>,
    pub foo443: Foo443<()>,
    pub foo444: Foo444<()>,
    pub foo445: Foo445<()>,
    pub foo446: Foo446<()>,
    pub foo447: Foo447<()>,
    pub foo448: Foo448<()>,
    pub foo449: Foo449<()>,
    pub foo450: Foo450<()>,
    pub foo451: Foo451<()>,
    pub foo452: Foo452<()>,
    pub foo453: Foo453<()>,
    pub foo454: Foo454<()>,
    pub foo455: Foo455<()>,
    pub foo456: Foo456<()>,
    pub foo457: Foo457<()>,
    pub foo458: Foo458<()>,
    pub foo459: Foo459<()>,
    pub foo460: Foo460<()>,
    pub foo461: Foo461<()>,
    pub foo462: Foo462<()>,
    pub foo463: Foo463<()>,
    pub foo464: Foo464<()>,
    pub foo465: Foo465<()>,
    pub foo466: Foo466<()>,
    pub foo467: Foo467<()>,
    pub foo468: Foo468<()>,
    pub foo469: Foo469<()>,
    pub foo470: Foo470<()>,
    pub foo471: Foo471<()>,
    pub foo472: Foo472<()>,
    pub foo473: Foo473<()>,
    pub foo474: Foo474<()>,
    pub foo475: Foo475<()>,
    pub foo476: Foo476<()>,
    pub foo477: Foo477<()>,
    pub foo478: Foo478<()>,
    pub foo479: Foo479<()>,
    pub foo480: Foo480<()>,
    pub foo481: Foo481<()>,
    pub foo482: Foo482<()>,
    pub foo483: Foo483<()>,
    pub foo484: Foo484<()>,
    pub foo485: Foo485<()>,
    pub foo486: Foo486<()>,
    pub foo487: Foo487<()>,
    pub foo488: Foo488<()>,
    pub foo489: Foo489<()>,
    pub foo490: Foo490<()>,
    pub foo491: Foo491<()>,
    pub foo492: Foo492<()>,
    pub foo493: Foo493<()>,
    pub foo494: Foo494<()>,
    pub foo495: Foo495<()>,
    pub foo496: Foo496<()>,
    pub foo497: Foo497<()>,
    pub foo498: Foo498<()>,
    pub foo499: Foo499<()>,
    pub foo500: Foo500<()>,
    pub foo501: Foo501<()>,
    pub foo502: Foo502<()>,
    pub foo503: Foo503<()>,
    pub foo504: Foo504<()>,
    pub foo505: Foo505<()>,
    pub foo506: Foo506<()>,
    pub foo507: Foo507<()>,
    pub foo508: Foo508<()>,
    pub foo509: Foo509<()>,
    pub foo510: Foo510<()>,
    pub foo511: Foo511<()>,
    pub foo512: Foo512<()>,
    pub foo513: Foo513<()>,
    pub foo514: Foo514<()>,
    pub foo515: Foo515<()>,
    pub foo516: Foo516<()>,
    pub foo517: Foo517<()>,
    pub foo518: Foo518<()>,
    pub foo519: Foo519<()>,
    pub foo520: Foo520<()>,
    pub foo521: Foo521<()>,
    pub foo522: Foo522<()>,
    pub foo523: Foo523<()>,
    pub foo524: Foo524<()>,
    pub foo525: Foo525<()>,
    pub foo526: Foo526<()>,
    pub foo527: Foo527<()>,
    pub foo528: Foo528<()>,
    pub foo529: Foo529<()>,
    pub foo530: Foo530<()>,
    pub foo531: Foo531<()>,
    pub foo532: Foo532<()>,
    pub foo533: Foo533<()>,
    pub foo534: Foo534<()>,
    pub foo535: Foo535<()>,
    pub foo536: Foo536<()>,
    pub foo537: Foo537<()>,
    pub foo538: Foo538<()>,
    pub foo539: Foo539<()>,
    pub foo540: Foo540<()>,
    pub foo541: Foo541<()>,
    pub foo542: Foo542<()>,
    pub foo543: Foo543<()>,
    pub foo544: Foo544<()>,
    pub foo545: Foo545<()>,
    pub foo546: Foo546<()>,
    pub foo547: Foo547<()>,
    pub foo548: Foo548<()>,
    pub foo549: Foo549<()>,
    pub foo550: Foo550<()>,
    pub foo551: Foo551<()>,
    pub foo552: Foo552<()>,
    pub foo553: Foo553<()>,
    pub foo554: Foo554<()>,
    pub foo555: Foo555<()>,
    pub foo556: Foo556<()>,
    pub foo557: Foo557<()>,
    pub foo558: Foo558<()>,
    pub foo559: Foo559<()>,
    pub foo560: Foo560<()>,
    pub foo561: Foo561<()>,
    pub foo562: Foo562<()>,
    pub foo563: Foo563<()>,
    pub foo564: Foo564<()>,
    pub foo565: Foo565<()>,
    pub foo566: Foo566<()>,
    pub foo567: Foo567<()>,
    pub foo568: Foo568<()>,
    pub foo569: Foo569<()>,
    pub foo570: Foo570<()>,
    pub foo571: Foo571<()>,
    pub foo572: Foo572<()>,
    pub foo573: Foo573<()>,
    pub foo574: Foo574<()>,
    pub foo575: Foo575<()>,
    pub foo576: Foo576<()>,
    pub foo577: Foo577<()>,
    pub foo578: Foo578<()>,
    pub foo579: Foo579<()>,
    pub foo580: Foo580<()>,
    pub foo581: Foo581<()>,
    pub foo582: Foo582<()>,
    pub foo583: Foo583<()>,
    pub foo584: Foo584<()>,
    pub foo585: Foo585<()>,
    pub foo586: Foo586<()>,
    pub foo587: Foo587<()>,
    pub foo588: Foo588<()>,
    pub foo589: Foo589<()>,
    pub foo590: Foo590<()>,
    pub foo591: Foo591<()>,
    pub foo592: Foo592<()>,
    pub foo593: Foo593<()>,
    pub foo594: Foo594<()>,
    pub foo595: Foo595<()>,
    pub foo596: Foo596<()>,
    pub foo597: Foo597<()>,
    pub foo598: Foo598<()>,
    pub foo599: Foo599<()>,
    pub foo600: Foo600<()>,
    pub foo601: Foo601<()>,
    pub foo602: Foo602<()>,
    pub foo603: Foo603<()>,
    pub foo604: Foo604<()>,
    pub foo605: Foo605<()>,
    pub foo606: Foo606<()>,
    pub foo607: Foo607<()>,
    pub foo608: Foo608<()>,
    pub foo609: Foo609<()>,
    pub foo610: Foo610<()>,
    pub foo611: Foo611<()>,
    pub foo612: Foo612<()>,
    pub foo613: Foo613<()>,
    pub foo614: Foo614<()>,
    pub foo615: Foo615<()>,
    pub foo616: Foo616<()>,
    pub foo617: Foo617<()>,
    pub foo618: Foo618<()>,
    pub foo619: Foo619<()>,
    pub foo620: Foo620<()>,
    pub foo621: Foo621<()>,
    pub foo622: Foo622<()>,
    pub foo623: Foo623<()>,
    pub foo624: Foo624<()>,
    pub foo625: Foo625<()>,
    pub foo626: Foo626<()>,
    pub foo627: Foo627<()>,
    pub foo628: Foo628<()>,
    pub foo629: Foo629<()>,
    pub foo630: Foo630<()>,
    pub foo631: Foo631<()>,
    pub foo632: Foo632<()>,
    pub foo633: Foo633<()>,
    pub foo634: Foo634<()>,
    pub foo635: Foo635<()>,
    pub foo636: Foo636<()>,
    pub foo637: Foo637<()>,
    pub foo638: Foo638<()>,
    pub foo639: Foo639<()>,
    pub foo640: Foo640<()>,
    pub foo641: Foo641<()>,
    pub foo642: Foo642<()>,
    pub foo643: Foo643<()>,
    pub foo644: Foo644<()>,
    pub foo645: Foo645<()>,
    pub foo646: Foo646<()>,
    pub foo647: Foo647<()>,
    pub foo648: Foo648<()>,
    pub foo649: Foo649<()>,
    pub foo650: Foo650<()>,
    pub foo651: Foo651<()>,
    pub foo652: Foo652<()>,
    pub foo653: Foo653<()>,
    pub foo654: Foo654<()>,
    pub foo655: Foo655<()>,
    pub foo656: Foo656<()>,
    pub foo657: Foo657<()>,
    pub foo658: Foo658<()>,
    pub foo659: Foo659<()>,
    pub foo660: Foo660<()>,
    pub foo661: Foo661<()>,
    pub foo662: Foo662<()>,
    pub foo663: Foo663<()>,
    pub foo664: Foo664<()>,
    pub foo665: Foo665<()>,
    pub foo666: Foo666<()>,
    pub foo667: Foo667<()>,
    pub foo668: Foo668<()>,
    pub foo669: Foo669<()>,
    pub foo670: Foo670<()>,
    pub foo671: Foo671<()>,
    pub foo672: Foo672<()>,
    pub foo673: Foo673<()>,
    pub foo674: Foo674<()>,
    pub foo675: Foo675<()>,
    pub foo676: Foo676<()>,
    pub foo677: Foo677<()>,
    pub foo678: Foo678<()>,
    pub foo679: Foo679<()>,
    pub foo680: Foo680<()>,
    pub foo681: Foo681<()>,
    pub foo682: Foo682<()>,
    pub foo683: Foo683<()>,
    pub foo684: Foo684<()>,
    pub foo685: Foo685<()>,
    pub foo686: Foo686<()>,
    pub foo687: Foo687<()>,
    pub foo688: Foo688<()>,
    pub foo689: Foo689<()>,
    pub foo690: Foo690<()>,
    pub foo691: Foo691<()>,
    pub foo692: Foo692<()>,
    pub foo693: Foo693<()>,
    pub foo694: Foo694<()>,
    pub foo695: Foo695<()>,
    pub foo696: Foo696<()>,
    pub foo697: Foo697<()>,
    pub foo698: Foo698<()>,
    pub foo699: Foo699<()>,
    pub foo700: Foo700<()>,
    pub foo701: Foo701<()>,
    pub foo702: Foo702<()>,
    pub foo703: Foo703<()>,
    pub foo704: Foo704<()>,
    pub foo705: Foo705<()>,
    pub foo706: Foo706<()>,
    pub foo707: Foo707<()>,
    pub foo708: Foo708<()>,
    pub foo709: Foo709<()>,
    pub foo710: Foo710<()>,
    pub foo711: Foo711<()>,
    pub foo712: Foo712<()>,
    pub foo713: Foo713<()>,
    pub foo714: Foo714<()>,
    pub foo715: Foo715<()>,
    pub foo716: Foo716<()>,
    pub foo717: Foo717<()>,
    pub foo718: Foo718<()>,
    pub foo719: Foo719<()>,
    pub foo720: Foo720<()>,
    pub foo721: Foo721<()>,
    pub foo722: Foo722<()>,
    pub foo723: Foo723<()>,
    pub foo724: Foo724<()>,
    pub foo725: Foo725<()>,
    pub foo726: Foo726<()>,
    pub foo727: Foo727<()>,
    pub foo728: Foo728<()>,
    pub foo729: Foo729<()>,
    pub foo730: Foo730<()>,
    pub foo731: Foo731<()>,
    pub foo732: Foo732<()>,
    pub foo733: Foo733<()>,
    pub foo734: Foo734<()>,
    pub foo735: Foo735<()>,
    pub foo736: Foo736<()>,
    pub foo737: Foo737<()>,
    pub foo738: Foo738<()>,
    pub foo739: Foo739<()>,
    pub foo740: Foo740<()>,
    pub foo741: Foo741<()>,
    pub foo742: Foo742<()>,
    pub foo743: Foo743<()>,
    pub foo744: Foo744<()>,
    pub foo745: Foo745<()>,
    pub foo746: Foo746<()>,
    pub foo747: Foo747<()>,
    pub foo748: Foo748<()>,
    pub foo749: Foo749<()>,
    pub foo750: Foo750<()>,
    pub foo751: Foo751<()>,
    pub foo752: Foo752<()>,
    pub foo753: Foo753<()>,
    pub foo754: Foo754<()>,
    pub foo755: Foo755<()>,
    pub foo756: Foo756<()>,
    pub foo757: Foo757<()>,
    pub foo758: Foo758<()>,
    pub foo759: Foo759<()>,
    pub foo760: Foo760<()>,
    pub foo761: Foo761<()>,
    pub foo762: Foo762<()>,
    pub foo763: Foo763<()>,
    pub foo764: Foo764<()>,
    pub foo765: Foo765<()>,
    pub foo766: Foo766<()>,
    pub foo767: Foo767<()>,
    pub foo768: Foo768<()>,
    pub foo769: Foo769<()>,
    pub foo770: Foo770<()>,
    pub foo771: Foo771<()>,
    pub foo772: Foo772<()>,
    pub foo773: Foo773<()>,
    pub foo774: Foo774<()>,
    pub foo775: Foo775<()>,
    pub foo776: Foo776<()>,
    pub foo777: Foo777<()>,
    pub foo778: Foo778<()>,
    pub foo779: Foo779<()>,
    pub foo780: Foo780<()>,
    pub foo781: Foo781<()>,
    pub foo782: Foo782<()>,
    pub foo783: Foo783<()>,
    pub foo784: Foo784<()>,
    pub foo785: Foo785<()>,
    pub foo786: Foo786<()>,
    pub foo787: Foo787<()>,
    pub foo788: Foo788<()>,
    pub foo789: Foo789<()>,
    pub foo790: Foo790<()>,
    pub foo791: Foo791<()>,
    pub foo792: Foo792<()>,
    pub foo793: Foo793<()>,
    pub foo794: Foo794<()>,
    pub foo795: Foo795<()>,
    pub foo796: Foo796<()>,
    pub foo797: Foo797<()>,
    pub foo798: Foo798<()>,
    pub foo799: Foo799<()>,
    pub foo800: Foo800<()>,
    pub foo801: Foo801<()>,
    pub foo802: Foo802<()>,
    pub foo803: Foo803<()>,
    pub foo804: Foo804<()>,
    pub foo805: Foo805<()>,
    pub foo806: Foo806<()>,
    pub foo807: Foo807<()>,
    pub foo808: Foo808<()>,
    pub foo809: Foo809<()>,
    pub foo810: Foo810<()>,
    pub foo811: Foo811<()>,
    pub foo812: Foo812<()>,
    pub foo813: Foo813<()>,
    pub foo814: Foo814<()>,
    pub foo815: Foo815<()>,
    pub foo816: Foo816<()>,
    pub foo817: Foo817<()>,
    pub foo818: Foo818<()>,
    pub foo819: Foo819<()>,
    pub foo820: Foo820<()>,
    pub foo821: Foo821<()>,
    pub foo822: Foo822<()>,
    pub foo823: Foo823<()>,
    pub foo824: Foo824<()>,
    pub foo825: Foo825<()>,
    pub foo826: Foo826<()>,
    pub foo827: Foo827<()>,
    pub foo828: Foo828<()>,
    pub foo829: Foo829<()>,
    pub foo830: Foo830<()>,
    pub foo831: Foo831<()>,
    pub foo832: Foo832<()>,
    pub foo833: Foo833<()>,
    pub foo834: Foo834<()>,
    pub foo835: Foo835<()>,
    pub foo836: Foo836<()>,
    pub foo837: Foo837<()>,
    pub foo838: Foo838<()>,
    pub foo839: Foo839<()>,
    pub foo840: Foo840<()>,
    pub foo841: Foo841<()>,
    pub foo842: Foo842<()>,
    pub foo843: Foo843<()>,
    pub foo844: Foo844<()>,
    pub foo845: Foo845<()>,
    pub foo846: Foo846<()>,
    pub foo847: Foo847<()>,
    pub foo848: Foo848<()>,
    pub foo849: Foo849<()>,
    pub foo850: Foo850<()>,
    pub foo851: Foo851<()>,
    pub foo852: Foo852<()>,
    pub foo853: Foo853<()>,
    pub foo854: Foo854<()>,
    pub foo855: Foo855<()>,
    pub foo856: Foo856<()>,
    pub foo857: Foo857<()>,
    pub foo858: Foo858<()>,
    pub foo859: Foo859<()>,
    pub foo860: Foo860<()>,
    pub foo861: Foo861<()>,
    pub foo862: Foo862<()>,
    pub foo863: Foo863<()>,
    pub foo864: Foo864<()>,
    pub foo865: Foo865<()>,
    pub foo866: Foo866<()>,
    pub foo867: Foo867<()>,
    pub foo868: Foo868<()>,
    pub foo869: Foo869<()>,
    pub foo870: Foo870<()>,
    pub foo871: Foo871<()>,
    pub foo872: Foo872<()>,
    pub foo873: Foo873<()>,
    pub foo874: Foo874<()>,
    pub foo875: Foo875<()>,
    pub foo876: Foo876<()>,
    pub foo877: Foo877<()>,
    pub foo878: Foo878<()>,
    pub foo879: Foo879<()>,
    pub foo880: Foo880<()>,
    pub foo881: Foo881<()>,
    pub foo882: Foo882<()>,
    pub foo883: Foo883<()>,
    pub foo884: Foo884<()>,
    pub foo885: Foo885<()>,
    pub foo886: Foo886<()>,
    pub foo887: Foo887<()>,
    pub foo888: Foo888<()>,
    pub foo889: Foo889<()>,
    pub foo890: Foo890<()>,
    pub foo891: Foo891<()>,
    pub foo892: Foo892<()>,
    pub foo893: Foo893<()>,
    pub foo894: Foo894<()>,
    pub foo895: Foo895<()>,
    pub foo896: Foo896<()>,
    pub foo897: Foo897<()>,
    pub foo898: Foo898<()>,
    pub foo899: Foo899<()>,
    pub foo900: Foo900<()>,
    pub foo901: Foo901<()>,
    pub foo902: Foo902<()>,
    pub foo903: Foo903<()>,
    pub foo904: Foo904<()>,
    pub foo905: Foo905<()>,
    pub foo906: Foo906<()>,
    pub foo907: Foo907<()>,
    pub foo908: Foo908<()>,
    pub foo909: Foo909<()>,
    pub foo910: Foo910<()>,
    pub foo911: Foo911<()>,
    pub foo912: Foo912<()>,
    pub foo913: Foo913<()>,
    pub foo914: Foo914<()>,
    pub foo915: Foo915<()>,
    pub foo916: Foo916<()>,
    pub foo917: Foo917<()>,
    pub foo918: Foo918<()>,
    pub foo919: Foo919<()>,
    pub foo920: Foo920<()>,
    pub foo921: Foo921<()>,
    pub foo922: Foo922<()>,
    pub foo923: Foo923<()>,
    pub foo924: Foo924<()>,
    pub foo925: Foo925<()>,
    pub foo926: Foo926<()>,
    pub foo927: Foo927<()>,
    pub foo928: Foo928<()>,
    pub foo929: Foo929<()>,
    pub foo930: Foo930<()>,
    pub foo931: Foo931<()>,
    pub foo932: Foo932<()>,
    pub foo933: Foo933<()>,
    pub foo934: Foo934<()>,
    pub foo935: Foo935<()>,
    pub foo936: Foo936<()>,
    pub foo937: Foo937<()>,
    pub foo938: Foo938<()>,
    pub foo939: Foo939<()>,
    pub foo940: Foo940<()>,
    pub foo941: Foo941<()>,
    pub foo942: Foo942<()>,
    pub foo943: Foo943<()>,
    pub foo944: Foo944<()>,
    pub foo945: Foo945<()>,
    pub foo946: Foo946<()>,
    pub foo947: Foo947<()>,
    pub foo948: Foo948<()>,
    pub foo949: Foo949<()>,
    pub foo950: Foo950<()>,
    pub foo951: Foo951<()>,
    pub foo952: Foo952<()>,
    pub foo953: Foo953<()>,
    pub foo954: Foo954<()>,
    pub foo955: Foo955<()>,
    pub foo956: Foo956<()>,
    pub foo957: Foo957<()>,
    pub foo958: Foo958<()>,
    pub foo959: Foo959<()>,
    pub foo960: Foo960<()>,
    pub foo961: Foo961<()>,
    pub foo962: Foo962<()>,
    pub foo963: Foo963<()>,
    pub foo964: Foo964<()>,
    pub foo965: Foo965<()>,
    pub foo966: Foo966<()>,
    pub foo967: Foo967<()>,
    pub foo968: Foo968<()>,
    pub foo969: Foo969<()>,
    pub foo970: Foo970<()>,
    pub foo971: Foo971<()>,
    pub foo972: Foo972<()>,
    pub foo973: Foo973<()>,
    pub foo974: Foo974<()>,
    pub foo975: Foo975<()>,
    pub foo976: Foo976<()>,
    pub foo977: Foo977<()>,
    pub foo978: Foo978<()>,
    pub foo979: Foo979<()>,
    pub foo980: Foo980<()>,
    pub foo981: Foo981<()>,
    pub foo982: Foo982<()>,
    pub foo983: Foo983<()>,
    pub foo984: Foo984<()>,
    pub foo985: Foo985<()>,
    pub foo986: Foo986<()>,
    pub foo987: Foo987<()>,
    pub foo988: Foo988<()>,
    pub foo989: Foo989<()>,
    pub foo990: Foo990<()>,
    pub foo991: Foo991<()>,
    pub foo992: Foo992<()>,
    pub foo993: Foo993<()>,
    pub foo994: Foo994<()>,
    pub foo995: Foo995<()>,
    pub foo996: Foo996<()>,
    pub foo997: Foo997<()>,
    pub foo998: Foo998<()>,
    pub foo999: Foo999<()>,
    pub foo1000: Foo1000<()>,
    pub foo1001: Foo1001<()>,
    pub foo1002: Foo1002<()>,
    pub foo1003: Foo1003<()>,
    pub foo1004: Foo1004<()>,
    pub foo1005: Foo1005<()>,
    pub foo1006: Foo1006<()>,
    pub foo1007: Foo1007<()>,
    pub foo1008: Foo1008<()>,
    pub foo1009: Foo1009<()>,
    pub foo1010: Foo1010<()>,
    pub foo1011: Foo1011<()>,
    pub foo1012: Foo1012<()>,
    pub foo1013: Foo1013<()>,
    pub foo1014: Foo1014<()>,
    pub foo1015: Foo1015<()>,
    pub foo1016: Foo1016<()>,
    pub foo1017: Foo1017<()>,
    pub foo1018: Foo1018<()>,
    pub foo1019: Foo1019<()>,
    pub foo1020: Foo1020<()>,
    pub foo1021: Foo1021<()>,
    pub foo1022: Foo1022<()>,
    pub foo1023: Foo1023<()>,
    pub foo1024: Foo1024<()>,
    pub foo1025: Foo1025<()>,
    pub foo1026: Foo1026<()>,
    pub foo1027: Foo1027<()>,
    pub foo1028: Foo1028<()>,
    pub foo1029: Foo1029<()>,
    pub foo1030: Foo1030<()>,
    pub foo1031: Foo1031<()>,
    pub foo1032: Foo1032<()>,
    pub foo1033: Foo1033<()>,
    pub foo1034: Foo1034<()>,
    pub foo1035: Foo1035<()>,
    pub foo1036: Foo1036<()>,
    pub foo1037: Foo1037<()>,
    pub foo1038: Foo1038<()>,
    pub foo1039: Foo1039<()>,
    pub foo1040: Foo1040<()>,
    pub foo1041: Foo1041<()>,
    pub foo1042: Foo1042<()>,
    pub foo1043: Foo1043<()>,
    pub foo1044: Foo1044<()>,
    pub foo1045: Foo1045<()>,
    pub foo1046: Foo1046<()>,
    pub foo1047: Foo1047<()>,
    pub foo1048: Foo1048<()>,
    pub foo1049: Foo1049<()>,
    pub foo1050: Foo1050<()>,
    pub foo1051: Foo1051<()>,
    pub foo1052: Foo1052<()>,
    pub foo1053: Foo1053<()>,
    pub foo1054: Foo1054<()>,
    pub foo1055: Foo1055<()>,
    pub foo1056: Foo1056<()>,
    pub foo1057: Foo1057<()>,
    pub foo1058: Foo1058<()>,
    pub foo1059: Foo1059<()>,
    pub foo1060: Foo1060<()>,
    pub foo1061: Foo1061<()>,
    pub foo1062: Foo1062<()>,
    pub foo1063: Foo1063<()>,
    pub foo1064: Foo1064<()>,
    pub foo1065: Foo1065<()>,
    pub foo1066: Foo1066<()>,
    pub foo1067: Foo1067<()>,
    pub foo1068: Foo1068<()>,
    pub foo1069: Foo1069<()>,
    pub foo1070: Foo1070<()>,
    pub foo1071: Foo1071<()>,
    pub foo1072: Foo1072<()>,
    pub foo1073: Foo1073<()>,
    pub foo1074: Foo1074<()>,
    pub foo1075: Foo1075<()>,
    pub foo1076: Foo1076<()>,
    pub foo1077: Foo1077<()>,
    pub foo1078: Foo1078<()>,
    pub foo1079: Foo1079<()>,
    pub foo1080: Foo1080<()>,
    pub foo1081: Foo1081<()>,
    pub foo1082: Foo1082<()>,
    pub foo1083: Foo1083<()>,
    pub foo1084: Foo1084<()>,
    pub foo1085: Foo1085<()>,
    pub foo1086: Foo1086<()>,
    pub foo1087: Foo1087<()>,
    pub foo1088: Foo1088<()>,
    pub foo1089: Foo1089<()>,
    pub foo1090: Foo1090<()>,
    pub foo1091: Foo1091<()>,
    pub foo1092: Foo1092<()>,
    pub foo1093: Foo1093<()>,
    pub foo1094: Foo1094<()>,
    pub foo1095: Foo1095<()>,
    pub foo1096: Foo1096<()>,
    pub foo1097: Foo1097<()>,
    pub foo1098: Foo1098<()>,
    pub foo1099: Foo1099<()>,
    pub foo1100: Foo1100<()>,
    pub foo1101: Foo1101<()>,
    pub foo1102: Foo1102<()>,
    pub foo1103: Foo1103<()>,
    pub foo1104: Foo1104<()>,
    pub foo1105: Foo1105<()>,
    pub foo1106: Foo1106<()>,
    pub foo1107: Foo1107<()>,
    pub foo1108: Foo1108<()>,
    pub foo1109: Foo1109<()>,
    pub foo1110: Foo1110<()>,
    pub foo1111: Foo1111<()>,
    pub foo1112: Foo1112<()>,
    pub foo1113: Foo1113<()>,
    pub foo1114: Foo1114<()>,
    pub foo1115: Foo1115<()>,
    pub foo1116: Foo1116<()>,
    pub foo1117: Foo1117<()>,
    pub foo1118: Foo1118<()>,
    pub foo1119: Foo1119<()>,
    pub foo1120: Foo1120<()>,
    pub foo1121: Foo1121<()>,
    pub foo1122: Foo1122<()>,
    pub foo1123: Foo1123<()>,
    pub foo1124: Foo1124<()>,
    pub foo1125: Foo1125<()>,
    pub foo1126: Foo1126<()>,
    pub foo1127: Foo1127<()>,
    pub foo1128: Foo1128<()>,
    pub foo1129: Foo1129<()>,
    pub foo1130: Foo1130<()>,
    pub foo1131: Foo1131<()>,
    pub foo1132: Foo1132<()>,
    pub foo1133: Foo1133<()>,
    pub foo1134: Foo1134<()>,
    pub foo1135: Foo1135<()>,
    pub foo1136: Foo1136<()>,
    pub foo1137: Foo1137<()>,
    pub foo1138: Foo1138<()>,
    pub foo1139: Foo1139<()>,
    pub foo1140: Foo1140<()>,
    pub foo1141: Foo1141<()>,
    pub foo1142: Foo1142<()>,
    pub foo1143: Foo1143<()>,
    pub foo1144: Foo1144<()>,
    pub foo1145: Foo1145<()>,
    pub foo1146: Foo1146<()>,
    pub foo1147: Foo1147<()>,
    pub foo1148: Foo1148<()>,
    pub foo1149: Foo1149<()>,
    pub foo1150: Foo1150<()>,
    pub foo1151: Foo1151<()>,
    pub foo1152: Foo1152<()>,
    pub foo1153: Foo1153<()>,
    pub foo1154: Foo1154<()>,
    pub foo1155: Foo1155<()>,
    pub foo1156: Foo1156<()>,
    pub foo1157: Foo1157<()>,
    pub foo1158: Foo1158<()>,
    pub foo1159: Foo1159<()>,
    pub foo1160: Foo1160<()>,
    pub foo1161: Foo1161<()>,
    pub foo1162: Foo1162<()>,
    pub foo1163: Foo1163<()>,
    pub foo1164: Foo1164<()>,
    pub foo1165: Foo1165<()>,
    pub foo1166: Foo1166<()>,
    pub foo1167: Foo1167<()>,
    pub foo1168: Foo1168<()>,
    pub foo1169: Foo1169<()>,
    pub foo1170: Foo1170<()>,
    pub foo1171: Foo1171<()>,
    pub foo1172: Foo1172<()>,
    pub foo1173: Foo1173<()>,
    pub foo1174: Foo1174<()>,
    pub foo1175: Foo1175<()>,
    pub foo1176: Foo1176<()>,
    pub foo1177: Foo1177<()>,
    pub foo1178: Foo1178<()>,
    pub foo1179: Foo1179<()>,
    pub foo1180: Foo1180<()>,
    pub foo1181: Foo1181<()>,
    pub foo1182: Foo1182<()>,
    pub foo1183: Foo1183<()>,
    pub foo1184: Foo1184<()>,
    pub foo1185: Foo1185<()>,
    pub foo1186: Foo1186<()>,
    pub foo1187: Foo1187<()>,
    pub foo1188: Foo1188<()>,
    pub foo1189: Foo1189<()>,
    pub foo1190: Foo1190<()>,
    pub foo1191: Foo1191<()>,
    pub foo1192: Foo1192<()>,
    pub foo1193: Foo1193<()>,
    pub foo1194: Foo1194<()>,
    pub foo1195: Foo1195<()>,
    pub foo1196: Foo1196<()>,
    pub foo1197: Foo1197<()>,
    pub foo1198: Foo1198<()>,
    pub foo1199: Foo1199<()>,
    pub foo1200: Foo1200<()>,
    pub foo1201: Foo1201<()>,
    pub foo1202: Foo1202<()>,
    pub foo1203: Foo1203<()>,
    pub foo1204: Foo1204<()>,
    pub foo1205: Foo1205<()>,
    pub foo1206: Foo1206<()>,
    pub foo1207: Foo1207<()>,
    pub foo1208: Foo1208<()>,
    pub foo1209: Foo1209<()>,
    pub foo1210: Foo1210<()>,
    pub foo1211: Foo1211<()>,
    pub foo1212: Foo1212<()>,
    pub foo1213: Foo1213<()>,
    pub foo1214: Foo1214<()>,
    pub foo1215: Foo1215<()>,
    pub foo1216: Foo1216<()>,
    pub foo1217: Foo1217<()>,
    pub foo1218: Foo1218<()>,
    pub foo1219: Foo1219<()>,
    pub foo1220: Foo1220<()>,
    pub foo1221: Foo1221<()>,
    pub foo1222: Foo1222<()>,
    pub foo1223: Foo1223<()>,
    pub foo1224: Foo1224<()>,
    pub foo1225: Foo1225<()>,
    pub foo1226: Foo1226<()>,
    pub foo1227: Foo1227<()>,
    pub foo1228: Foo1228<()>,
    pub foo1229: Foo1229<()>,
    pub foo1230: Foo1230<()>,
    pub foo1231: Foo1231<()>,
    pub foo1232: Foo1232<()>,
    pub foo1233: Foo1233<()>,
    pub foo1234: Foo1234<()>,
    pub foo1235: Foo1235<()>,
    pub foo1236: Foo1236<()>,
    pub foo1237: Foo1237<()>,
    pub foo1238: Foo1238<()>,
    pub foo1239: Foo1239<()>,
    pub foo1240: Foo1240<()>,
    pub foo1241: Foo1241<()>,
    pub foo1242: Foo1242<()>,
    pub foo1243: Foo1243<()>,
    pub foo1244: Foo1244<()>,
    pub foo1245: Foo1245<()>,
    pub foo1246: Foo1246<()>,
    pub foo1247: Foo1247<()>,
    pub foo1248: Foo1248<()>,
    pub foo1249: Foo1249<()>,
    pub foo1250: Foo1250<()>,
    pub foo1251: Foo1251<()>,
    pub foo1252: Foo1252<()>,
    pub foo1253: Foo1253<()>,
    pub foo1254: Foo1254<()>,
    pub foo1255: Foo1255<()>,
    pub foo1256: Foo1256<()>,
    pub foo1257: Foo1257<()>,
    pub foo1258: Foo1258<()>,
    pub foo1259: Foo1259<()>,
    pub foo1260: Foo1260<()>,
    pub foo1261: Foo1261<()>,
    pub foo1262: Foo1262<()>,
    pub foo1263: Foo1263<()>,
    pub foo1264: Foo1264<()>,
    pub foo1265: Foo1265<()>,
    pub foo1266: Foo1266<()>,
    pub foo1267: Foo1267<()>,
    pub foo1268: Foo1268<()>,
    pub foo1269: Foo1269<()>,
    pub foo1270: Foo1270<()>,
    pub foo1271: Foo1271<()>,
    pub foo1272: Foo1272<()>,
    pub foo1273: Foo1273<()>,
    pub foo1274: Foo1274<()>,
    pub foo1275: Foo1275<()>,
    pub foo1276: Foo1276<()>,
    pub foo1277: Foo1277<()>,
    pub foo1278: Foo1278<()>,
    pub foo1279: Foo1279<()>,
    pub foo1280: Foo1280<()>,
    pub foo1281: Foo1281<()>,
    pub foo1282: Foo1282<()>,
    pub foo1283: Foo1283<()>,
    pub foo1284: Foo1284<()>,
    pub foo1285: Foo1285<()>,
    pub foo1286: Foo1286<()>,
    pub foo1287: Foo1287<()>,
    pub foo1288: Foo1288<()>,
    pub foo1289: Foo1289<()>,
    pub foo1290: Foo1290<()>,
    pub foo1291: Foo1291<()>,
    pub foo1292: Foo1292<()>,
    pub foo1293: Foo1293<()>,
    pub foo1294: Foo1294<()>,
    pub foo1295: Foo1295<()>,
    pub foo1296: Foo1296<()>,
    pub foo1297: Foo1297<()>,
    pub foo1298: Foo1298<()>,
    pub foo1299: Foo1299<()>,
    pub foo1300: Foo1300<()>,
    pub foo1301: Foo1301<()>,
    pub foo1302: Foo1302<()>,
    pub foo1303: Foo1303<()>,
    pub foo1304: Foo1304<()>,
    pub foo1305: Foo1305<()>,
    pub foo1306: Foo1306<()>,
    pub foo1307: Foo1307<()>,
    pub foo1308: Foo1308<()>,
    pub foo1309: Foo1309<()>,
    pub foo1310: Foo1310<()>,
    pub foo1311: Foo1311<()>,
    pub foo1312: Foo1312<()>,
    pub foo1313: Foo1313<()>,
    pub foo1314: Foo1314<()>,
    pub foo1315: Foo1315<()>,
    pub foo1316: Foo1316<()>,
    pub foo1317: Foo1317<()>,
    pub foo1318: Foo1318<()>,
    pub foo1319: Foo1319<()>,
    pub foo1320: Foo1320<()>,
    pub foo1321: Foo1321<()>,
    pub foo1322: Foo1322<()>,
    pub foo1323: Foo1323<()>,
    pub foo1324: Foo1324<()>,
    pub foo1325: Foo1325<()>,
    pub foo1326: Foo1326<()>,
    pub foo1327: Foo1327<()>,
    pub foo1328: Foo1328<()>,
    pub foo1329: Foo1329<()>,
    pub foo1330: Foo1330<()>,
    pub foo1331: Foo1331<()>,
    pub foo1332: Foo1332<()>,
    pub foo1333: Foo1333<()>,
    pub foo1334: Foo1334<()>,
    pub foo1335: Foo1335<()>,
    pub foo1336: Foo1336<()>,
    pub foo1337: Foo1337<()>,
    pub foo1338: Foo1338<()>,
    pub foo1339: Foo1339<()>,
    pub foo1340: Foo1340<()>,
    pub foo1341: Foo1341<()>,
    pub foo1342: Foo1342<()>,
    pub foo1343: Foo1343<()>,
    pub foo1344: Foo1344<()>,
    pub foo1345: Foo1345<()>,
    pub foo1346: Foo1346<()>,
    pub foo1347: Foo1347<()>,
    pub foo1348: Foo1348<()>,
    pub foo1349: Foo1349<()>,
    pub foo1350: Foo1350<()>,
    pub foo1351: Foo1351<()>,
    pub foo1352: Foo1352<()>,
    pub foo1353: Foo1353<()>,
    pub foo1354: Foo1354<()>,
    pub foo1355: Foo1355<()>,
    pub foo1356: Foo1356<()>,
    pub foo1357: Foo1357<()>,
    pub foo1358: Foo1358<()>,
    pub foo1359: Foo1359<()>,
    pub foo1360: Foo1360<()>,
    pub foo1361: Foo1361<()>,
    pub foo1362: Foo1362<()>,
    pub foo1363: Foo1363<()>,
    pub foo1364: Foo1364<()>,
    pub foo1365: Foo1365<()>,
    pub foo1366: Foo1366<()>,
    pub foo1367: Foo1367<()>,
    pub foo1368: Foo1368<()>,
    pub foo1369: Foo1369<()>,
    pub foo1370: Foo1370<()>,
    pub foo1371: Foo1371<()>,
    pub foo1372: Foo1372<()>,
    pub foo1373: Foo1373<()>,
    pub foo1374: Foo1374<()>,
    pub foo1375: Foo1375<()>,
    pub foo1376: Foo1376<()>,
    pub foo1377: Foo1377<()>,
    pub foo1378: Foo1378<()>,
    pub foo1379: Foo1379<()>,
    pub foo1380: Foo1380<()>,
    pub foo1381: Foo1381<()>,
    pub foo1382: Foo1382<()>,
    pub foo1383: Foo1383<()>,
    pub foo1384: Foo1384<()>,
    pub foo1385: Foo1385<()>,
    pub foo1386: Foo1386<()>,
    pub foo1387: Foo1387<()>,
    pub foo1388: Foo1388<()>,
    pub foo1389: Foo1389<()>,
    pub foo1390: Foo1390<()>,
    pub foo1391: Foo1391<()>,
    pub foo1392: Foo1392<()>,
    pub foo1393: Foo1393<()>,
    pub foo1394: Foo1394<()>,
    pub foo1395: Foo1395<()>,
    pub foo1396: Foo1396<()>,
    pub foo1397: Foo1397<()>,
    pub foo1398: Foo1398<()>,
    pub foo1399: Foo1399<()>,
    pub foo1400: Foo1400<()>,
    pub foo1401: Foo1401<()>,
    pub foo1402: Foo1402<()>,
    pub foo1403: Foo1403<()>,
    pub foo1404: Foo1404<()>,
    pub foo1405: Foo1405<()>,
    pub foo1406: Foo1406<()>,
    pub foo1407: Foo1407<()>,
    pub foo1408: Foo1408<()>,
    pub foo1409: Foo1409<()>,
    pub foo1410: Foo1410<()>,
    pub foo1411: Foo1411<()>,
    pub foo1412: Foo1412<()>,
    pub foo1413: Foo1413<()>,
    pub foo1414: Foo1414<()>,
    pub foo1415: Foo1415<()>,
    pub foo1416: Foo1416<()>,
    pub foo1417: Foo1417<()>,
    pub foo1418: Foo1418<()>,
    pub foo1419: Foo1419<()>,
    pub foo1420: Foo1420<()>,
    pub foo1421: Foo1421<()>,
    pub foo1422: Foo1422<()>,
    pub foo1423: Foo1423<()>,
    pub foo1424: Foo1424<()>,
    pub foo1425: Foo1425<()>,
    pub foo1426: Foo1426<()>,
    pub foo1427: Foo1427<()>,
    pub foo1428: Foo1428<()>,
    pub foo1429: Foo1429<()>,
    pub foo1430: Foo1430<()>,
    pub foo1431: Foo1431<()>,
    pub foo1432: Foo1432<()>,
    pub foo1433: Foo1433<()>,
    pub foo1434: Foo1434<()>,
    pub foo1435: Foo1435<()>,
    pub foo1436: Foo1436<()>,
    pub foo1437: Foo1437<()>,
    pub foo1438: Foo1438<()>,
    pub foo1439: Foo1439<()>,
    pub foo1440: Foo1440<()>,
    pub foo1441: Foo1441<()>,
    pub foo1442: Foo1442<()>,
    pub foo1443: Foo1443<()>,
    pub foo1444: Foo1444<()>,
    pub foo1445: Foo1445<()>,
    pub foo1446: Foo1446<()>,
    pub foo1447: Foo1447<()>,
    pub foo1448: Foo1448<()>,
    pub foo1449: Foo1449<()>,
    pub foo1450: Foo1450<()>,
    pub foo1451: Foo1451<()>,
    pub foo1452: Foo1452<()>,
    pub foo1453: Foo1453<()>,
    pub foo1454: Foo1454<()>,
    pub foo1455: Foo1455<()>,
    pub foo1456: Foo1456<()>,
    pub foo1457: Foo1457<()>,
    pub foo1458: Foo1458<()>,
    pub foo1459: Foo1459<()>,
    pub foo1460: Foo1460<()>,
    pub foo1461: Foo1461<()>,
    pub foo1462: Foo1462<()>,
    pub foo1463: Foo1463<()>,
    pub foo1464: Foo1464<()>,
    pub foo1465: Foo1465<()>,
    pub foo1466: Foo1466<()>,
    pub foo1467: Foo1467<()>,
    pub foo1468: Foo1468<()>,
    pub foo1469: Foo1469<()>,
    pub foo1470: Foo1470<()>,
    pub foo1471: Foo1471<()>,
    pub foo1472: Foo1472<()>,
    pub foo1473: Foo1473<()>,
    pub foo1474: Foo1474<()>,
    pub foo1475: Foo1475<()>,
    pub foo1476: Foo1476<()>,
    pub foo1477: Foo1477<()>,
    pub foo1478: Foo1478<()>,
    pub foo1479: Foo1479<()>,
    pub foo1480: Foo1480<()>,
    pub foo1481: Foo1481<()>,
    pub foo1482: Foo1482<()>,
    pub foo1483: Foo1483<()>,
    pub foo1484: Foo1484<()>,
    pub foo1485: Foo1485<()>,
    pub foo1486: Foo1486<()>,
    pub foo1487: Foo1487<()>,
    pub foo1488: Foo1488<()>,
    pub foo1489: Foo1489<()>,
    pub foo1490: Foo1490<()>,
    pub foo1491: Foo1491<()>,
    pub foo1492: Foo1492<()>,
    pub foo1493: Foo1493<()>,
    pub foo1494: Foo1494<()>,
    pub foo1495: Foo1495<()>,
    pub foo1496: Foo1496<()>,
    pub foo1497: Foo1497<()>,
    pub foo1498: Foo1498<()>,
    pub foo1499: Foo1499<()>,
    pub foo1500: Foo1500<()>,
}

impl Tokens {
    pub fn new() -> Self {
        Self {
            foo1: Foo1::new(),
            foo2: Foo2::new(),
            foo3: Foo3::new(),
            foo4: Foo4::new(),
            foo5: Foo5::new(),
            foo6: Foo6::new(),
            foo7: Foo7::new(),
            foo8: Foo8::new(),
            foo9: Foo9::new(),
            foo10: Foo10::new(),
            foo11: Foo11::new(),
            foo12: Foo12::new(),
            foo13: Foo13::new(),
            foo14: Foo14::new(),
            foo15: Foo15::new(),
            foo16: Foo16::new(),
            foo17: Foo17::new(),
            foo18: Foo18::new(),
            foo19: Foo19::new(),
            foo20: Foo20::new(),
            foo21: Foo21::new(),
            foo22: Foo22::new(),
            foo23: Foo23::new(),
            foo24: Foo24::new(),
            foo25: Foo25::new(),
            foo26: Foo26::new(),
            foo27: Foo27::new(),
            foo28: Foo28::new(),
            foo29: Foo29::new(),
            foo30: Foo30::new(),
            foo31: Foo31::new(),
            foo32: Foo32::new(),
            foo33: Foo33::new(),
            foo34: Foo34::new(),
            foo35: Foo35::new(),
            foo36: Foo36::new(),
            foo37: Foo37::new(),
            foo38: Foo38::new(),
            foo39: Foo39::new(),
            foo40: Foo40::new(),
            foo41: Foo41::new(),
            foo42: Foo42::new(),
            foo43: Foo43::new(),
            foo44: Foo44::new(),
            foo45: Foo45::new(),
            foo46: Foo46::new(),
            foo47: Foo47::new(),
            foo48: Foo48::new(),
            foo49: Foo49::new(),
            foo50: Foo50::new(),
            foo51: Foo51::new(),
            foo52: Foo52::new(),
            foo53: Foo53::new(),
            foo54: Foo54::new(),
            foo55: Foo55::new(),
            foo56: Foo56::new(),
            foo57: Foo57::new(),
            foo58: Foo58::new(),
            foo59: Foo59::new(),
            foo60: Foo60::new(),
            foo61: Foo61::new(),
            foo62: Foo62::new(),
            foo63: Foo63::new(),
            foo64: Foo64::new(),
            foo65: Foo65::new(),
            foo66: Foo66::new(),
            foo67: Foo67::new(),
            foo68: Foo68::new(),
            foo69: Foo69::new(),
            foo70: Foo70::new(),
            foo71: Foo71::new(),
            foo72: Foo72::new(),
            foo73: Foo73::new(),
            foo74: Foo74::new(),
            foo75: Foo75::new(),
            foo76: Foo76::new(),
            foo77: Foo77::new(),
            foo78: Foo78::new(),
            foo79: Foo79::new(),
            foo80: Foo80::new(),
            foo81: Foo81::new(),
            foo82: Foo82::new(),
            foo83: Foo83::new(),
            foo84: Foo84::new(),
            foo85: Foo85::new(),
            foo86: Foo86::new(),
            foo87: Foo87::new(),
            foo88: Foo88::new(),
            foo89: Foo89::new(),
            foo90: Foo90::new(),
            foo91: Foo91::new(),
            foo92: Foo92::new(),
            foo93: Foo93::new(),
            foo94: Foo94::new(),
            foo95: Foo95::new(),
            foo96: Foo96::new(),
            foo97: Foo97::new(),
            foo98: Foo98::new(),
            foo99: Foo99::new(),
            foo100: Foo100::new(),
            foo101: Foo101::new(),
            foo102: Foo102::new(),
            foo103: Foo103::new(),
            foo104: Foo104::new(),
            foo105: Foo105::new(),
            foo106: Foo106::new(),
            foo107: Foo107::new(),
            foo108: Foo108::new(),
            foo109: Foo109::new(),
            foo110: Foo110::new(),
            foo111: Foo111::new(),
            foo112: Foo112::new(),
            foo113: Foo113::new(),
            foo114: Foo114::new(),
            foo115: Foo115::new(),
            foo116: Foo116::new(),
            foo117: Foo117::new(),
            foo118: Foo118::new(),
            foo119: Foo119::new(),
            foo120: Foo120::new(),
            foo121: Foo121::new(),
            foo122: Foo122::new(),
            foo123: Foo123::new(),
            foo124: Foo124::new(),
            foo125: Foo125::new(),
            foo126: Foo126::new(),
            foo127: Foo127::new(),
            foo128: Foo128::new(),
            foo129: Foo129::new(),
            foo130: Foo130::new(),
            foo131: Foo131::new(),
            foo132: Foo132::new(),
            foo133: Foo133::new(),
            foo134: Foo134::new(),
            foo135: Foo135::new(),
            foo136: Foo136::new(),
            foo137: Foo137::new(),
            foo138: Foo138::new(),
            foo139: Foo139::new(),
            foo140: Foo140::new(),
            foo141: Foo141::new(),
            foo142: Foo142::new(),
            foo143: Foo143::new(),
            foo144: Foo144::new(),
            foo145: Foo145::new(),
            foo146: Foo146::new(),
            foo147: Foo147::new(),
            foo148: Foo148::new(),
            foo149: Foo149::new(),
            foo150: Foo150::new(),
            foo151: Foo151::new(),
            foo152: Foo152::new(),
            foo153: Foo153::new(),
            foo154: Foo154::new(),
            foo155: Foo155::new(),
            foo156: Foo156::new(),
            foo157: Foo157::new(),
            foo158: Foo158::new(),
            foo159: Foo159::new(),
            foo160: Foo160::new(),
            foo161: Foo161::new(),
            foo162: Foo162::new(),
            foo163: Foo163::new(),
            foo164: Foo164::new(),
            foo165: Foo165::new(),
            foo166: Foo166::new(),
            foo167: Foo167::new(),
            foo168: Foo168::new(),
            foo169: Foo169::new(),
            foo170: Foo170::new(),
            foo171: Foo171::new(),
            foo172: Foo172::new(),
            foo173: Foo173::new(),
            foo174: Foo174::new(),
            foo175: Foo175::new(),
            foo176: Foo176::new(),
            foo177: Foo177::new(),
            foo178: Foo178::new(),
            foo179: Foo179::new(),
            foo180: Foo180::new(),
            foo181: Foo181::new(),
            foo182: Foo182::new(),
            foo183: Foo183::new(),
            foo184: Foo184::new(),
            foo185: Foo185::new(),
            foo186: Foo186::new(),
            foo187: Foo187::new(),
            foo188: Foo188::new(),
            foo189: Foo189::new(),
            foo190: Foo190::new(),
            foo191: Foo191::new(),
            foo192: Foo192::new(),
            foo193: Foo193::new(),
            foo194: Foo194::new(),
            foo195: Foo195::new(),
            foo196: Foo196::new(),
            foo197: Foo197::new(),
            foo198: Foo198::new(),
            foo199: Foo199::new(),
            foo200: Foo200::new(),
            foo201: Foo201::new(),
            foo202: Foo202::new(),
            foo203: Foo203::new(),
            foo204: Foo204::new(),
            foo205: Foo205::new(),
            foo206: Foo206::new(),
            foo207: Foo207::new(),
            foo208: Foo208::new(),
            foo209: Foo209::new(),
            foo210: Foo210::new(),
            foo211: Foo211::new(),
            foo212: Foo212::new(),
            foo213: Foo213::new(),
            foo214: Foo214::new(),
            foo215: Foo215::new(),
            foo216: Foo216::new(),
            foo217: Foo217::new(),
            foo218: Foo218::new(),
            foo219: Foo219::new(),
            foo220: Foo220::new(),
            foo221: Foo221::new(),
            foo222: Foo222::new(),
            foo223: Foo223::new(),
            foo224: Foo224::new(),
            foo225: Foo225::new(),
            foo226: Foo226::new(),
            foo227: Foo227::new(),
            foo228: Foo228::new(),
            foo229: Foo229::new(),
            foo230: Foo230::new(),
            foo231: Foo231::new(),
            foo232: Foo232::new(),
            foo233: Foo233::new(),
            foo234: Foo234::new(),
            foo235: Foo235::new(),
            foo236: Foo236::new(),
            foo237: Foo237::new(),
            foo238: Foo238::new(),
            foo239: Foo239::new(),
            foo240: Foo240::new(),
            foo241: Foo241::new(),
            foo242: Foo242::new(),
            foo243: Foo243::new(),
            foo244: Foo244::new(),
            foo245: Foo245::new(),
            foo246: Foo246::new(),
            foo247: Foo247::new(),
            foo248: Foo248::new(),
            foo249: Foo249::new(),
            foo250: Foo250::new(),
            foo251: Foo251::new(),
            foo252: Foo252::new(),
            foo253: Foo253::new(),
            foo254: Foo254::new(),
            foo255: Foo255::new(),
            foo256: Foo256::new(),
            foo257: Foo257::new(),
            foo258: Foo258::new(),
            foo259: Foo259::new(),
            foo260: Foo260::new(),
            foo261: Foo261::new(),
            foo262: Foo262::new(),
            foo263: Foo263::new(),
            foo264: Foo264::new(),
            foo265: Foo265::new(),
            foo266: Foo266::new(),
            foo267: Foo267::new(),
            foo268: Foo268::new(),
            foo269: Foo269::new(),
            foo270: Foo270::new(),
            foo271: Foo271::new(),
            foo272: Foo272::new(),
            foo273: Foo273::new(),
            foo274: Foo274::new(),
            foo275: Foo275::new(),
            foo276: Foo276::new(),
            foo277: Foo277::new(),
            foo278: Foo278::new(),
            foo279: Foo279::new(),
            foo280: Foo280::new(),
            foo281: Foo281::new(),
            foo282: Foo282::new(),
            foo283: Foo283::new(),
            foo284: Foo284::new(),
            foo285: Foo285::new(),
            foo286: Foo286::new(),
            foo287: Foo287::new(),
            foo288: Foo288::new(),
            foo289: Foo289::new(),
            foo290: Foo290::new(),
            foo291: Foo291::new(),
            foo292: Foo292::new(),
            foo293: Foo293::new(),
            foo294: Foo294::new(),
            foo295: Foo295::new(),
            foo296: Foo296::new(),
            foo297: Foo297::new(),
            foo298: Foo298::new(),
            foo299: Foo299::new(),
            foo300: Foo300::new(),
            foo301: Foo301::new(),
            foo302: Foo302::new(),
            foo303: Foo303::new(),
            foo304: Foo304::new(),
            foo305: Foo305::new(),
            foo306: Foo306::new(),
            foo307: Foo307::new(),
            foo308: Foo308::new(),
            foo309: Foo309::new(),
            foo310: Foo310::new(),
            foo311: Foo311::new(),
            foo312: Foo312::new(),
            foo313: Foo313::new(),
            foo314: Foo314::new(),
            foo315: Foo315::new(),
            foo316: Foo316::new(),
            foo317: Foo317::new(),
            foo318: Foo318::new(),
            foo319: Foo319::new(),
            foo320: Foo320::new(),
            foo321: Foo321::new(),
            foo322: Foo322::new(),
            foo323: Foo323::new(),
            foo324: Foo324::new(),
            foo325: Foo325::new(),
            foo326: Foo326::new(),
            foo327: Foo327::new(),
            foo328: Foo328::new(),
            foo329: Foo329::new(),
            foo330: Foo330::new(),
            foo331: Foo331::new(),
            foo332: Foo332::new(),
            foo333: Foo333::new(),
            foo334: Foo334::new(),
            foo335: Foo335::new(),
            foo336: Foo336::new(),
            foo337: Foo337::new(),
            foo338: Foo338::new(),
            foo339: Foo339::new(),
            foo340: Foo340::new(),
            foo341: Foo341::new(),
            foo342: Foo342::new(),
            foo343: Foo343::new(),
            foo344: Foo344::new(),
            foo345: Foo345::new(),
            foo346: Foo346::new(),
            foo347: Foo347::new(),
            foo348: Foo348::new(),
            foo349: Foo349::new(),
            foo350: Foo350::new(),
            foo351: Foo351::new(),
            foo352: Foo352::new(),
            foo353: Foo353::new(),
            foo354: Foo354::new(),
            foo355: Foo355::new(),
            foo356: Foo356::new(),
            foo357: Foo357::new(),
            foo358: Foo358::new(),
            foo359: Foo359::new(),
            foo360: Foo360::new(),
            foo361: Foo361::new(),
            foo362: Foo362::new(),
            foo363: Foo363::new(),
            foo364: Foo364::new(),
            foo365: Foo365::new(),
            foo366: Foo366::new(),
            foo367: Foo367::new(),
            foo368: Foo368::new(),
            foo369: Foo369::new(),
            foo370: Foo370::new(),
            foo371: Foo371::new(),
            foo372: Foo372::new(),
            foo373: Foo373::new(),
            foo374: Foo374::new(),
            foo375: Foo375::new(),
            foo376: Foo376::new(),
            foo377: Foo377::new(),
            foo378: Foo378::new(),
            foo379: Foo379::new(),
            foo380: Foo380::new(),
            foo381: Foo381::new(),
            foo382: Foo382::new(),
            foo383: Foo383::new(),
            foo384: Foo384::new(),
            foo385: Foo385::new(),
            foo386: Foo386::new(),
            foo387: Foo387::new(),
            foo388: Foo388::new(),
            foo389: Foo389::new(),
            foo390: Foo390::new(),
            foo391: Foo391::new(),
            foo392: Foo392::new(),
            foo393: Foo393::new(),
            foo394: Foo394::new(),
            foo395: Foo395::new(),
            foo396: Foo396::new(),
            foo397: Foo397::new(),
            foo398: Foo398::new(),
            foo399: Foo399::new(),
            foo400: Foo400::new(),
            foo401: Foo401::new(),
            foo402: Foo402::new(),
            foo403: Foo403::new(),
            foo404: Foo404::new(),
            foo405: Foo405::new(),
            foo406: Foo406::new(),
            foo407: Foo407::new(),
            foo408: Foo408::new(),
            foo409: Foo409::new(),
            foo410: Foo410::new(),
            foo411: Foo411::new(),
            foo412: Foo412::new(),
            foo413: Foo413::new(),
            foo414: Foo414::new(),
            foo415: Foo415::new(),
            foo416: Foo416::new(),
            foo417: Foo417::new(),
            foo418: Foo418::new(),
            foo419: Foo419::new(),
            foo420: Foo420::new(),
            foo421: Foo421::new(),
            foo422: Foo422::new(),
            foo423: Foo423::new(),
            foo424: Foo424::new(),
            foo425: Foo425::new(),
            foo426: Foo426::new(),
            foo427: Foo427::new(),
            foo428: Foo428::new(),
            foo429: Foo429::new(),
            foo430: Foo430::new(),
            foo431: Foo431::new(),
            foo432: Foo432::new(),
            foo433: Foo433::new(),
            foo434: Foo434::new(),
            foo435: Foo435::new(),
            foo436: Foo436::new(),
            foo437: Foo437::new(),
            foo438: Foo438::new(),
            foo439: Foo439::new(),
            foo440: Foo440::new(),
            foo441: Foo441::new(),
            foo442: Foo442::new(),
            foo443: Foo443::new(),
            foo444: Foo444::new(),
            foo445: Foo445::new(),
            foo446: Foo446::new(),
            foo447: Foo447::new(),
            foo448: Foo448::new(),
            foo449: Foo449::new(),
            foo450: Foo450::new(),
            foo451: Foo451::new(),
            foo452: Foo452::new(),
            foo453: Foo453::new(),
            foo454: Foo454::new(),
            foo455: Foo455::new(),
            foo456: Foo456::new(),
            foo457: Foo457::new(),
            foo458: Foo458::new(),
            foo459: Foo459::new(),
            foo460: Foo460::new(),
            foo461: Foo461::new(),
            foo462: Foo462::new(),
            foo463: Foo463::new(),
            foo464: Foo464::new(),
            foo465: Foo465::new(),
            foo466: Foo466::new(),
            foo467: Foo467::new(),
            foo468: Foo468::new(),
            foo469: Foo469::new(),
            foo470: Foo470::new(),
            foo471: Foo471::new(),
            foo472: Foo472::new(),
            foo473: Foo473::new(),
            foo474: Foo474::new(),
            foo475: Foo475::new(),
            foo476: Foo476::new(),
            foo477: Foo477::new(),
            foo478: Foo478::new(),
            foo479: Foo479::new(),
            foo480: Foo480::new(),
            foo481: Foo481::new(),
            foo482: Foo482::new(),
            foo483: Foo483::new(),
            foo484: Foo484::new(),
            foo485: Foo485::new(),
            foo486: Foo486::new(),
            foo487: Foo487::new(),
            foo488: Foo488::new(),
            foo489: Foo489::new(),
            foo490: Foo490::new(),
            foo491: Foo491::new(),
            foo492: Foo492::new(),
            foo493: Foo493::new(),
            foo494: Foo494::new(),
            foo495: Foo495::new(),
            foo496: Foo496::new(),
            foo497: Foo497::new(),
            foo498: Foo498::new(),
            foo499: Foo499::new(),
            foo500: Foo500::new(),
            foo501: Foo501::new(),
            foo502: Foo502::new(),
            foo503: Foo503::new(),
            foo504: Foo504::new(),
            foo505: Foo505::new(),
            foo506: Foo506::new(),
            foo507: Foo507::new(),
            foo508: Foo508::new(),
            foo509: Foo509::new(),
            foo510: Foo510::new(),
            foo511: Foo511::new(),
            foo512: Foo512::new(),
            foo513: Foo513::new(),
            foo514: Foo514::new(),
            foo515: Foo515::new(),
            foo516: Foo516::new(),
            foo517: Foo517::new(),
            foo518: Foo518::new(),
            foo519: Foo519::new(),
            foo520: Foo520::new(),
            foo521: Foo521::new(),
            foo522: Foo522::new(),
            foo523: Foo523::new(),
            foo524: Foo524::new(),
            foo525: Foo525::new(),
            foo526: Foo526::new(),
            foo527: Foo527::new(),
            foo528: Foo528::new(),
            foo529: Foo529::new(),
            foo530: Foo530::new(),
            foo531: Foo531::new(),
            foo532: Foo532::new(),
            foo533: Foo533::new(),
            foo534: Foo534::new(),
            foo535: Foo535::new(),
            foo536: Foo536::new(),
            foo537: Foo537::new(),
            foo538: Foo538::new(),
            foo539: Foo539::new(),
            foo540: Foo540::new(),
            foo541: Foo541::new(),
            foo542: Foo542::new(),
            foo543: Foo543::new(),
            foo544: Foo544::new(),
            foo545: Foo545::new(),
            foo546: Foo546::new(),
            foo547: Foo547::new(),
            foo548: Foo548::new(),
            foo549: Foo549::new(),
            foo550: Foo550::new(),
            foo551: Foo551::new(),
            foo552: Foo552::new(),
            foo553: Foo553::new(),
            foo554: Foo554::new(),
            foo555: Foo555::new(),
            foo556: Foo556::new(),
            foo557: Foo557::new(),
            foo558: Foo558::new(),
            foo559: Foo559::new(),
            foo560: Foo560::new(),
            foo561: Foo561::new(),
            foo562: Foo562::new(),
            foo563: Foo563::new(),
            foo564: Foo564::new(),
            foo565: Foo565::new(),
            foo566: Foo566::new(),
            foo567: Foo567::new(),
            foo568: Foo568::new(),
            foo569: Foo569::new(),
            foo570: Foo570::new(),
            foo571: Foo571::new(),
            foo572: Foo572::new(),
            foo573: Foo573::new(),
            foo574: Foo574::new(),
            foo575: Foo575::new(),
            foo576: Foo576::new(),
            foo577: Foo577::new(),
            foo578: Foo578::new(),
            foo579: Foo579::new(),
            foo580: Foo580::new(),
            foo581: Foo581::new(),
            foo582: Foo582::new(),
            foo583: Foo583::new(),
            foo584: Foo584::new(),
            foo585: Foo585::new(),
            foo586: Foo586::new(),
            foo587: Foo587::new(),
            foo588: Foo588::new(),
            foo589: Foo589::new(),
            foo590: Foo590::new(),
            foo591: Foo591::new(),
            foo592: Foo592::new(),
            foo593: Foo593::new(),
            foo594: Foo594::new(),
            foo595: Foo595::new(),
            foo596: Foo596::new(),
            foo597: Foo597::new(),
            foo598: Foo598::new(),
            foo599: Foo599::new(),
            foo600: Foo600::new(),
            foo601: Foo601::new(),
            foo602: Foo602::new(),
            foo603: Foo603::new(),
            foo604: Foo604::new(),
            foo605: Foo605::new(),
            foo606: Foo606::new(),
            foo607: Foo607::new(),
            foo608: Foo608::new(),
            foo609: Foo609::new(),
            foo610: Foo610::new(),
            foo611: Foo611::new(),
            foo612: Foo612::new(),
            foo613: Foo613::new(),
            foo614: Foo614::new(),
            foo615: Foo615::new(),
            foo616: Foo616::new(),
            foo617: Foo617::new(),
            foo618: Foo618::new(),
            foo619: Foo619::new(),
            foo620: Foo620::new(),
            foo621: Foo621::new(),
            foo622: Foo622::new(),
            foo623: Foo623::new(),
            foo624: Foo624::new(),
            foo625: Foo625::new(),
            foo626: Foo626::new(),
            foo627: Foo627::new(),
            foo628: Foo628::new(),
            foo629: Foo629::new(),
            foo630: Foo630::new(),
            foo631: Foo631::new(),
            foo632: Foo632::new(),
            foo633: Foo633::new(),
            foo634: Foo634::new(),
            foo635: Foo635::new(),
            foo636: Foo636::new(),
            foo637: Foo637::new(),
            foo638: Foo638::new(),
            foo639: Foo639::new(),
            foo640: Foo640::new(),
            foo641: Foo641::new(),
            foo642: Foo642::new(),
            foo643: Foo643::new(),
            foo644: Foo644::new(),
            foo645: Foo645::new(),
            foo646: Foo646::new(),
            foo647: Foo647::new(),
            foo648: Foo648::new(),
            foo649: Foo649::new(),
            foo650: Foo650::new(),
            foo651: Foo651::new(),
            foo652: Foo652::new(),
            foo653: Foo653::new(),
            foo654: Foo654::new(),
            foo655: Foo655::new(),
            foo656: Foo656::new(),
            foo657: Foo657::new(),
            foo658: Foo658::new(),
            foo659: Foo659::new(),
            foo660: Foo660::new(),
            foo661: Foo661::new(),
            foo662: Foo662::new(),
            foo663: Foo663::new(),
            foo664: Foo664::new(),
            foo665: Foo665::new(),
            foo666: Foo666::new(),
            foo667: Foo667::new(),
            foo668: Foo668::new(),
            foo669: Foo669::new(),
            foo670: Foo670::new(),
            foo671: Foo671::new(),
            foo672: Foo672::new(),
            foo673: Foo673::new(),
            foo674: Foo674::new(),
            foo675: Foo675::new(),
            foo676: Foo676::new(),
            foo677: Foo677::new(),
            foo678: Foo678::new(),
            foo679: Foo679::new(),
            foo680: Foo680::new(),
            foo681: Foo681::new(),
            foo682: Foo682::new(),
            foo683: Foo683::new(),
            foo684: Foo684::new(),
            foo685: Foo685::new(),
            foo686: Foo686::new(),
            foo687: Foo687::new(),
            foo688: Foo688::new(),
            foo689: Foo689::new(),
            foo690: Foo690::new(),
            foo691: Foo691::new(),
            foo692: Foo692::new(),
            foo693: Foo693::new(),
            foo694: Foo694::new(),
            foo695: Foo695::new(),
            foo696: Foo696::new(),
            foo697: Foo697::new(),
            foo698: Foo698::new(),
            foo699: Foo699::new(),
            foo700: Foo700::new(),
            foo701: Foo701::new(),
            foo702: Foo702::new(),
            foo703: Foo703::new(),
            foo704: Foo704::new(),
            foo705: Foo705::new(),
            foo706: Foo706::new(),
            foo707: Foo707::new(),
            foo708: Foo708::new(),
            foo709: Foo709::new(),
            foo710: Foo710::new(),
            foo711: Foo711::new(),
            foo712: Foo712::new(),
            foo713: Foo713::new(),
            foo714: Foo714::new(),
            foo715: Foo715::new(),
            foo716: Foo716::new(),
            foo717: Foo717::new(),
            foo718: Foo718::new(),
            foo719: Foo719::new(),
            foo720: Foo720::new(),
            foo721: Foo721::new(),
            foo722: Foo722::new(),
            foo723: Foo723::new(),
            foo724: Foo724::new(),
            foo725: Foo725::new(),
            foo726: Foo726::new(),
            foo727: Foo727::new(),
            foo728: Foo728::new(),
            foo729: Foo729::new(),
            foo730: Foo730::new(),
            foo731: Foo731::new(),
            foo732: Foo732::new(),
            foo733: Foo733::new(),
            foo734: Foo734::new(),
            foo735: Foo735::new(),
            foo736: Foo736::new(),
            foo737: Foo737::new(),
            foo738: Foo738::new(),
            foo739: Foo739::new(),
            foo740: Foo740::new(),
            foo741: Foo741::new(),
            foo742: Foo742::new(),
            foo743: Foo743::new(),
            foo744: Foo744::new(),
            foo745: Foo745::new(),
            foo746: Foo746::new(),
            foo747: Foo747::new(),
            foo748: Foo748::new(),
            foo749: Foo749::new(),
            foo750: Foo750::new(),
            foo751: Foo751::new(),
            foo752: Foo752::new(),
            foo753: Foo753::new(),
            foo754: Foo754::new(),
            foo755: Foo755::new(),
            foo756: Foo756::new(),
            foo757: Foo757::new(),
            foo758: Foo758::new(),
            foo759: Foo759::new(),
            foo760: Foo760::new(),
            foo761: Foo761::new(),
            foo762: Foo762::new(),
            foo763: Foo763::new(),
            foo764: Foo764::new(),
            foo765: Foo765::new(),
            foo766: Foo766::new(),
            foo767: Foo767::new(),
            foo768: Foo768::new(),
            foo769: Foo769::new(),
            foo770: Foo770::new(),
            foo771: Foo771::new(),
            foo772: Foo772::new(),
            foo773: Foo773::new(),
            foo774: Foo774::new(),
            foo775: Foo775::new(),
            foo776: Foo776::new(),
            foo777: Foo777::new(),
            foo778: Foo778::new(),
            foo779: Foo779::new(),
            foo780: Foo780::new(),
            foo781: Foo781::new(),
            foo782: Foo782::new(),
            foo783: Foo783::new(),
            foo784: Foo784::new(),
            foo785: Foo785::new(),
            foo786: Foo786::new(),
            foo787: Foo787::new(),
            foo788: Foo788::new(),
            foo789: Foo789::new(),
            foo790: Foo790::new(),
            foo791: Foo791::new(),
            foo792: Foo792::new(),
            foo793: Foo793::new(),
            foo794: Foo794::new(),
            foo795: Foo795::new(),
            foo796: Foo796::new(),
            foo797: Foo797::new(),
            foo798: Foo798::new(),
            foo799: Foo799::new(),
            foo800: Foo800::new(),
            foo801: Foo801::new(),
            foo802: Foo802::new(),
            foo803: Foo803::new(),
            foo804: Foo804::new(),
            foo805: Foo805::new(),
            foo806: Foo806::new(),
            foo807: Foo807::new(),
            foo808: Foo808::new(),
            foo809: Foo809::new(),
            foo810: Foo810::new(),
            foo811: Foo811::new(),
            foo812: Foo812::new(),
            foo813: Foo813::new(),
            foo814: Foo814::new(),
            foo815: Foo815::new(),
            foo816: Foo816::new(),
            foo817: Foo817::new(),
            foo818: Foo818::new(),
            foo819: Foo819::new(),
            foo820: Foo820::new(),
            foo821: Foo821::new(),
            foo822: Foo822::new(),
            foo823: Foo823::new(),
            foo824: Foo824::new(),
            foo825: Foo825::new(),
            foo826: Foo826::new(),
            foo827: Foo827::new(),
            foo828: Foo828::new(),
            foo829: Foo829::new(),
            foo830: Foo830::new(),
            foo831: Foo831::new(),
            foo832: Foo832::new(),
            foo833: Foo833::new(),
            foo834: Foo834::new(),
            foo835: Foo835::new(),
            foo836: Foo836::new(),
            foo837: Foo837::new(),
            foo838: Foo838::new(),
            foo839: Foo839::new(),
            foo840: Foo840::new(),
            foo841: Foo841::new(),
            foo842: Foo842::new(),
            foo843: Foo843::new(),
            foo844: Foo844::new(),
            foo845: Foo845::new(),
            foo846: Foo846::new(),
            foo847: Foo847::new(),
            foo848: Foo848::new(),
            foo849: Foo849::new(),
            foo850: Foo850::new(),
            foo851: Foo851::new(),
            foo852: Foo852::new(),
            foo853: Foo853::new(),
            foo854: Foo854::new(),
            foo855: Foo855::new(),
            foo856: Foo856::new(),
            foo857: Foo857::new(),
            foo858: Foo858::new(),
            foo859: Foo859::new(),
            foo860: Foo860::new(),
            foo861: Foo861::new(),
            foo862: Foo862::new(),
            foo863: Foo863::new(),
            foo864: Foo864::new(),
            foo865: Foo865::new(),
            foo866: Foo866::new(),
            foo867: Foo867::new(),
            foo868: Foo868::new(),
            foo869: Foo869::new(),
            foo870: Foo870::new(),
            foo871: Foo871::new(),
            foo872: Foo872::new(),
            foo873: Foo873::new(),
            foo874: Foo874::new(),
            foo875: Foo875::new(),
            foo876: Foo876::new(),
            foo877: Foo877::new(),
            foo878: Foo878::new(),
            foo879: Foo879::new(),
            foo880: Foo880::new(),
            foo881: Foo881::new(),
            foo882: Foo882::new(),
            foo883: Foo883::new(),
            foo884: Foo884::new(),
            foo885: Foo885::new(),
            foo886: Foo886::new(),
            foo887: Foo887::new(),
            foo888: Foo888::new(),
            foo889: Foo889::new(),
            foo890: Foo890::new(),
            foo891: Foo891::new(),
            foo892: Foo892::new(),
            foo893: Foo893::new(),
            foo894: Foo894::new(),
            foo895: Foo895::new(),
            foo896: Foo896::new(),
            foo897: Foo897::new(),
            foo898: Foo898::new(),
            foo899: Foo899::new(),
            foo900: Foo900::new(),
            foo901: Foo901::new(),
            foo902: Foo902::new(),
            foo903: Foo903::new(),
            foo904: Foo904::new(),
            foo905: Foo905::new(),
            foo906: Foo906::new(),
            foo907: Foo907::new(),
            foo908: Foo908::new(),
            foo909: Foo909::new(),
            foo910: Foo910::new(),
            foo911: Foo911::new(),
            foo912: Foo912::new(),
            foo913: Foo913::new(),
            foo914: Foo914::new(),
            foo915: Foo915::new(),
            foo916: Foo916::new(),
            foo917: Foo917::new(),
            foo918: Foo918::new(),
            foo919: Foo919::new(),
            foo920: Foo920::new(),
            foo921: Foo921::new(),
            foo922: Foo922::new(),
            foo923: Foo923::new(),
            foo924: Foo924::new(),
            foo925: Foo925::new(),
            foo926: Foo926::new(),
            foo927: Foo927::new(),
            foo928: Foo928::new(),
            foo929: Foo929::new(),
            foo930: Foo930::new(),
            foo931: Foo931::new(),
            foo932: Foo932::new(),
            foo933: Foo933::new(),
            foo934: Foo934::new(),
            foo935: Foo935::new(),
            foo936: Foo936::new(),
            foo937: Foo937::new(),
            foo938: Foo938::new(),
            foo939: Foo939::new(),
            foo940: Foo940::new(),
            foo941: Foo941::new(),
            foo942: Foo942::new(),
            foo943: Foo943::new(),
            foo944: Foo944::new(),
            foo945: Foo945::new(),
            foo946: Foo946::new(),
            foo947: Foo947::new(),
            foo948: Foo948::new(),
            foo949: Foo949::new(),
            foo950: Foo950::new(),
            foo951: Foo951::new(),
            foo952: Foo952::new(),
            foo953: Foo953::new(),
            foo954: Foo954::new(),
            foo955: Foo955::new(),
            foo956: Foo956::new(),
            foo957: Foo957::new(),
            foo958: Foo958::new(),
            foo959: Foo959::new(),
            foo960: Foo960::new(),
            foo961: Foo961::new(),
            foo962: Foo962::new(),
            foo963: Foo963::new(),
            foo964: Foo964::new(),
            foo965: Foo965::new(),
            foo966: Foo966::new(),
            foo967: Foo967::new(),
            foo968: Foo968::new(),
            foo969: Foo969::new(),
            foo970: Foo970::new(),
            foo971: Foo971::new(),
            foo972: Foo972::new(),
            foo973: Foo973::new(),
            foo974: Foo974::new(),
            foo975: Foo975::new(),
            foo976: Foo976::new(),
            foo977: Foo977::new(),
            foo978: Foo978::new(),
            foo979: Foo979::new(),
            foo980: Foo980::new(),
            foo981: Foo981::new(),
            foo982: Foo982::new(),
            foo983: Foo983::new(),
            foo984: Foo984::new(),
            foo985: Foo985::new(),
            foo986: Foo986::new(),
            foo987: Foo987::new(),
            foo988: Foo988::new(),
            foo989: Foo989::new(),
            foo990: Foo990::new(),
            foo991: Foo991::new(),
            foo992: Foo992::new(),
            foo993: Foo993::new(),
            foo994: Foo994::new(),
            foo995: Foo995::new(),
            foo996: Foo996::new(),
            foo997: Foo997::new(),
            foo998: Foo998::new(),
            foo999: Foo999::new(),
            foo1000: Foo1000::new(),
            foo1001: Foo1001::new(),
            foo1002: Foo1002::new(),
            foo1003: Foo1003::new(),
            foo1004: Foo1004::new(),
            foo1005: Foo1005::new(),
            foo1006: Foo1006::new(),
            foo1007: Foo1007::new(),
            foo1008: Foo1008::new(),
            foo1009: Foo1009::new(),
            foo1010: Foo1010::new(),
            foo1011: Foo1011::new(),
            foo1012: Foo1012::new(),
            foo1013: Foo1013::new(),
            foo1014: Foo1014::new(),
            foo1015: Foo1015::new(),
            foo1016: Foo1016::new(),
            foo1017: Foo1017::new(),
            foo1018: Foo1018::new(),
            foo1019: Foo1019::new(),
            foo1020: Foo1020::new(),
            foo1021: Foo1021::new(),
            foo1022: Foo1022::new(),
            foo1023: Foo1023::new(),
            foo1024: Foo1024::new(),
            foo1025: Foo1025::new(),
            foo1026: Foo1026::new(),
            foo1027: Foo1027::new(),
            foo1028: Foo1028::new(),
            foo1029: Foo1029::new(),
            foo1030: Foo1030::new(),
            foo1031: Foo1031::new(),
            foo1032: Foo1032::new(),
            foo1033: Foo1033::new(),
            foo1034: Foo1034::new(),
            foo1035: Foo1035::new(),
            foo1036: Foo1036::new(),
            foo1037: Foo1037::new(),
            foo1038: Foo1038::new(),
            foo1039: Foo1039::new(),
            foo1040: Foo1040::new(),
            foo1041: Foo1041::new(),
            foo1042: Foo1042::new(),
            foo1043: Foo1043::new(),
            foo1044: Foo1044::new(),
            foo1045: Foo1045::new(),
            foo1046: Foo1046::new(),
            foo1047: Foo1047::new(),
            foo1048: Foo1048::new(),
            foo1049: Foo1049::new(),
            foo1050: Foo1050::new(),
            foo1051: Foo1051::new(),
            foo1052: Foo1052::new(),
            foo1053: Foo1053::new(),
            foo1054: Foo1054::new(),
            foo1055: Foo1055::new(),
            foo1056: Foo1056::new(),
            foo1057: Foo1057::new(),
            foo1058: Foo1058::new(),
            foo1059: Foo1059::new(),
            foo1060: Foo1060::new(),
            foo1061: Foo1061::new(),
            foo1062: Foo1062::new(),
            foo1063: Foo1063::new(),
            foo1064: Foo1064::new(),
            foo1065: Foo1065::new(),
            foo1066: Foo1066::new(),
            foo1067: Foo1067::new(),
            foo1068: Foo1068::new(),
            foo1069: Foo1069::new(),
            foo1070: Foo1070::new(),
            foo1071: Foo1071::new(),
            foo1072: Foo1072::new(),
            foo1073: Foo1073::new(),
            foo1074: Foo1074::new(),
            foo1075: Foo1075::new(),
            foo1076: Foo1076::new(),
            foo1077: Foo1077::new(),
            foo1078: Foo1078::new(),
            foo1079: Foo1079::new(),
            foo1080: Foo1080::new(),
            foo1081: Foo1081::new(),
            foo1082: Foo1082::new(),
            foo1083: Foo1083::new(),
            foo1084: Foo1084::new(),
            foo1085: Foo1085::new(),
            foo1086: Foo1086::new(),
            foo1087: Foo1087::new(),
            foo1088: Foo1088::new(),
            foo1089: Foo1089::new(),
            foo1090: Foo1090::new(),
            foo1091: Foo1091::new(),
            foo1092: Foo1092::new(),
            foo1093: Foo1093::new(),
            foo1094: Foo1094::new(),
            foo1095: Foo1095::new(),
            foo1096: Foo1096::new(),
            foo1097: Foo1097::new(),
            foo1098: Foo1098::new(),
            foo1099: Foo1099::new(),
            foo1100: Foo1100::new(),
            foo1101: Foo1101::new(),
            foo1102: Foo1102::new(),
            foo1103: Foo1103::new(),
            foo1104: Foo1104::new(),
            foo1105: Foo1105::new(),
            foo1106: Foo1106::new(),
            foo1107: Foo1107::new(),
            foo1108: Foo1108::new(),
            foo1109: Foo1109::new(),
            foo1110: Foo1110::new(),
            foo1111: Foo1111::new(),
            foo1112: Foo1112::new(),
            foo1113: Foo1113::new(),
            foo1114: Foo1114::new(),
            foo1115: Foo1115::new(),
            foo1116: Foo1116::new(),
            foo1117: Foo1117::new(),
            foo1118: Foo1118::new(),
            foo1119: Foo1119::new(),
            foo1120: Foo1120::new(),
            foo1121: Foo1121::new(),
            foo1122: Foo1122::new(),
            foo1123: Foo1123::new(),
            foo1124: Foo1124::new(),
            foo1125: Foo1125::new(),
            foo1126: Foo1126::new(),
            foo1127: Foo1127::new(),
            foo1128: Foo1128::new(),
            foo1129: Foo1129::new(),
            foo1130: Foo1130::new(),
            foo1131: Foo1131::new(),
            foo1132: Foo1132::new(),
            foo1133: Foo1133::new(),
            foo1134: Foo1134::new(),
            foo1135: Foo1135::new(),
            foo1136: Foo1136::new(),
            foo1137: Foo1137::new(),
            foo1138: Foo1138::new(),
            foo1139: Foo1139::new(),
            foo1140: Foo1140::new(),
            foo1141: Foo1141::new(),
            foo1142: Foo1142::new(),
            foo1143: Foo1143::new(),
            foo1144: Foo1144::new(),
            foo1145: Foo1145::new(),
            foo1146: Foo1146::new(),
            foo1147: Foo1147::new(),
            foo1148: Foo1148::new(),
            foo1149: Foo1149::new(),
            foo1150: Foo1150::new(),
            foo1151: Foo1151::new(),
            foo1152: Foo1152::new(),
            foo1153: Foo1153::new(),
            foo1154: Foo1154::new(),
            foo1155: Foo1155::new(),
            foo1156: Foo1156::new(),
            foo1157: Foo1157::new(),
            foo1158: Foo1158::new(),
            foo1159: Foo1159::new(),
            foo1160: Foo1160::new(),
            foo1161: Foo1161::new(),
            foo1162: Foo1162::new(),
            foo1163: Foo1163::new(),
            foo1164: Foo1164::new(),
            foo1165: Foo1165::new(),
            foo1166: Foo1166::new(),
            foo1167: Foo1167::new(),
            foo1168: Foo1168::new(),
            foo1169: Foo1169::new(),
            foo1170: Foo1170::new(),
            foo1171: Foo1171::new(),
            foo1172: Foo1172::new(),
            foo1173: Foo1173::new(),
            foo1174: Foo1174::new(),
            foo1175: Foo1175::new(),
            foo1176: Foo1176::new(),
            foo1177: Foo1177::new(),
            foo1178: Foo1178::new(),
            foo1179: Foo1179::new(),
            foo1180: Foo1180::new(),
            foo1181: Foo1181::new(),
            foo1182: Foo1182::new(),
            foo1183: Foo1183::new(),
            foo1184: Foo1184::new(),
            foo1185: Foo1185::new(),
            foo1186: Foo1186::new(),
            foo1187: Foo1187::new(),
            foo1188: Foo1188::new(),
            foo1189: Foo1189::new(),
            foo1190: Foo1190::new(),
            foo1191: Foo1191::new(),
            foo1192: Foo1192::new(),
            foo1193: Foo1193::new(),
            foo1194: Foo1194::new(),
            foo1195: Foo1195::new(),
            foo1196: Foo1196::new(),
            foo1197: Foo1197::new(),
            foo1198: Foo1198::new(),
            foo1199: Foo1199::new(),
            foo1200: Foo1200::new(),
            foo1201: Foo1201::new(),
            foo1202: Foo1202::new(),
            foo1203: Foo1203::new(),
            foo1204: Foo1204::new(),
            foo1205: Foo1205::new(),
            foo1206: Foo1206::new(),
            foo1207: Foo1207::new(),
            foo1208: Foo1208::new(),
            foo1209: Foo1209::new(),
            foo1210: Foo1210::new(),
            foo1211: Foo1211::new(),
            foo1212: Foo1212::new(),
            foo1213: Foo1213::new(),
            foo1214: Foo1214::new(),
            foo1215: Foo1215::new(),
            foo1216: Foo1216::new(),
            foo1217: Foo1217::new(),
            foo1218: Foo1218::new(),
            foo1219: Foo1219::new(),
            foo1220: Foo1220::new(),
            foo1221: Foo1221::new(),
            foo1222: Foo1222::new(),
            foo1223: Foo1223::new(),
            foo1224: Foo1224::new(),
            foo1225: Foo1225::new(),
            foo1226: Foo1226::new(),
            foo1227: Foo1227::new(),
            foo1228: Foo1228::new(),
            foo1229: Foo1229::new(),
            foo1230: Foo1230::new(),
            foo1231: Foo1231::new(),
            foo1232: Foo1232::new(),
            foo1233: Foo1233::new(),
            foo1234: Foo1234::new(),
            foo1235: Foo1235::new(),
            foo1236: Foo1236::new(),
            foo1237: Foo1237::new(),
            foo1238: Foo1238::new(),
            foo1239: Foo1239::new(),
            foo1240: Foo1240::new(),
            foo1241: Foo1241::new(),
            foo1242: Foo1242::new(),
            foo1243: Foo1243::new(),
            foo1244: Foo1244::new(),
            foo1245: Foo1245::new(),
            foo1246: Foo1246::new(),
            foo1247: Foo1247::new(),
            foo1248: Foo1248::new(),
            foo1249: Foo1249::new(),
            foo1250: Foo1250::new(),
            foo1251: Foo1251::new(),
            foo1252: Foo1252::new(),
            foo1253: Foo1253::new(),
            foo1254: Foo1254::new(),
            foo1255: Foo1255::new(),
            foo1256: Foo1256::new(),
            foo1257: Foo1257::new(),
            foo1258: Foo1258::new(),
            foo1259: Foo1259::new(),
            foo1260: Foo1260::new(),
            foo1261: Foo1261::new(),
            foo1262: Foo1262::new(),
            foo1263: Foo1263::new(),
            foo1264: Foo1264::new(),
            foo1265: Foo1265::new(),
            foo1266: Foo1266::new(),
            foo1267: Foo1267::new(),
            foo1268: Foo1268::new(),
            foo1269: Foo1269::new(),
            foo1270: Foo1270::new(),
            foo1271: Foo1271::new(),
            foo1272: Foo1272::new(),
            foo1273: Foo1273::new(),
            foo1274: Foo1274::new(),
            foo1275: Foo1275::new(),
            foo1276: Foo1276::new(),
            foo1277: Foo1277::new(),
            foo1278: Foo1278::new(),
            foo1279: Foo1279::new(),
            foo1280: Foo1280::new(),
            foo1281: Foo1281::new(),
            foo1282: Foo1282::new(),
            foo1283: Foo1283::new(),
            foo1284: Foo1284::new(),
            foo1285: Foo1285::new(),
            foo1286: Foo1286::new(),
            foo1287: Foo1287::new(),
            foo1288: Foo1288::new(),
            foo1289: Foo1289::new(),
            foo1290: Foo1290::new(),
            foo1291: Foo1291::new(),
            foo1292: Foo1292::new(),
            foo1293: Foo1293::new(),
            foo1294: Foo1294::new(),
            foo1295: Foo1295::new(),
            foo1296: Foo1296::new(),
            foo1297: Foo1297::new(),
            foo1298: Foo1298::new(),
            foo1299: Foo1299::new(),
            foo1300: Foo1300::new(),
            foo1301: Foo1301::new(),
            foo1302: Foo1302::new(),
            foo1303: Foo1303::new(),
            foo1304: Foo1304::new(),
            foo1305: Foo1305::new(),
            foo1306: Foo1306::new(),
            foo1307: Foo1307::new(),
            foo1308: Foo1308::new(),
            foo1309: Foo1309::new(),
            foo1310: Foo1310::new(),
            foo1311: Foo1311::new(),
            foo1312: Foo1312::new(),
            foo1313: Foo1313::new(),
            foo1314: Foo1314::new(),
            foo1315: Foo1315::new(),
            foo1316: Foo1316::new(),
            foo1317: Foo1317::new(),
            foo1318: Foo1318::new(),
            foo1319: Foo1319::new(),
            foo1320: Foo1320::new(),
            foo1321: Foo1321::new(),
            foo1322: Foo1322::new(),
            foo1323: Foo1323::new(),
            foo1324: Foo1324::new(),
            foo1325: Foo1325::new(),
            foo1326: Foo1326::new(),
            foo1327: Foo1327::new(),
            foo1328: Foo1328::new(),
            foo1329: Foo1329::new(),
            foo1330: Foo1330::new(),
            foo1331: Foo1331::new(),
            foo1332: Foo1332::new(),
            foo1333: Foo1333::new(),
            foo1334: Foo1334::new(),
            foo1335: Foo1335::new(),
            foo1336: Foo1336::new(),
            foo1337: Foo1337::new(),
            foo1338: Foo1338::new(),
            foo1339: Foo1339::new(),
            foo1340: Foo1340::new(),
            foo1341: Foo1341::new(),
            foo1342: Foo1342::new(),
            foo1343: Foo1343::new(),
            foo1344: Foo1344::new(),
            foo1345: Foo1345::new(),
            foo1346: Foo1346::new(),
            foo1347: Foo1347::new(),
            foo1348: Foo1348::new(),
            foo1349: Foo1349::new(),
            foo1350: Foo1350::new(),
            foo1351: Foo1351::new(),
            foo1352: Foo1352::new(),
            foo1353: Foo1353::new(),
            foo1354: Foo1354::new(),
            foo1355: Foo1355::new(),
            foo1356: Foo1356::new(),
            foo1357: Foo1357::new(),
            foo1358: Foo1358::new(),
            foo1359: Foo1359::new(),
            foo1360: Foo1360::new(),
            foo1361: Foo1361::new(),
            foo1362: Foo1362::new(),
            foo1363: Foo1363::new(),
            foo1364: Foo1364::new(),
            foo1365: Foo1365::new(),
            foo1366: Foo1366::new(),
            foo1367: Foo1367::new(),
            foo1368: Foo1368::new(),
            foo1369: Foo1369::new(),
            foo1370: Foo1370::new(),
            foo1371: Foo1371::new(),
            foo1372: Foo1372::new(),
            foo1373: Foo1373::new(),
            foo1374: Foo1374::new(),
            foo1375: Foo1375::new(),
            foo1376: Foo1376::new(),
            foo1377: Foo1377::new(),
            foo1378: Foo1378::new(),
            foo1379: Foo1379::new(),
            foo1380: Foo1380::new(),
            foo1381: Foo1381::new(),
            foo1382: Foo1382::new(),
            foo1383: Foo1383::new(),
            foo1384: Foo1384::new(),
            foo1385: Foo1385::new(),
            foo1386: Foo1386::new(),
            foo1387: Foo1387::new(),
            foo1388: Foo1388::new(),
            foo1389: Foo1389::new(),
            foo1390: Foo1390::new(),
            foo1391: Foo1391::new(),
            foo1392: Foo1392::new(),
            foo1393: Foo1393::new(),
            foo1394: Foo1394::new(),
            foo1395: Foo1395::new(),
            foo1396: Foo1396::new(),
            foo1397: Foo1397::new(),
            foo1398: Foo1398::new(),
            foo1399: Foo1399::new(),
            foo1400: Foo1400::new(),
            foo1401: Foo1401::new(),
            foo1402: Foo1402::new(),
            foo1403: Foo1403::new(),
            foo1404: Foo1404::new(),
            foo1405: Foo1405::new(),
            foo1406: Foo1406::new(),
            foo1407: Foo1407::new(),
            foo1408: Foo1408::new(),
            foo1409: Foo1409::new(),
            foo1410: Foo1410::new(),
            foo1411: Foo1411::new(),
            foo1412: Foo1412::new(),
            foo1413: Foo1413::new(),
            foo1414: Foo1414::new(),
            foo1415: Foo1415::new(),
            foo1416: Foo1416::new(),
            foo1417: Foo1417::new(),
            foo1418: Foo1418::new(),
            foo1419: Foo1419::new(),
            foo1420: Foo1420::new(),
            foo1421: Foo1421::new(),
            foo1422: Foo1422::new(),
            foo1423: Foo1423::new(),
            foo1424: Foo1424::new(),
            foo1425: Foo1425::new(),
            foo1426: Foo1426::new(),
            foo1427: Foo1427::new(),
            foo1428: Foo1428::new(),
            foo1429: Foo1429::new(),
            foo1430: Foo1430::new(),
            foo1431: Foo1431::new(),
            foo1432: Foo1432::new(),
            foo1433: Foo1433::new(),
            foo1434: Foo1434::new(),
            foo1435: Foo1435::new(),
            foo1436: Foo1436::new(),
            foo1437: Foo1437::new(),
            foo1438: Foo1438::new(),
            foo1439: Foo1439::new(),
            foo1440: Foo1440::new(),
            foo1441: Foo1441::new(),
            foo1442: Foo1442::new(),
            foo1443: Foo1443::new(),
            foo1444: Foo1444::new(),
            foo1445: Foo1445::new(),
            foo1446: Foo1446::new(),
            foo1447: Foo1447::new(),
            foo1448: Foo1448::new(),
            foo1449: Foo1449::new(),
            foo1450: Foo1450::new(),
            foo1451: Foo1451::new(),
            foo1452: Foo1452::new(),
            foo1453: Foo1453::new(),
            foo1454: Foo1454::new(),
            foo1455: Foo1455::new(),
            foo1456: Foo1456::new(),
            foo1457: Foo1457::new(),
            foo1458: Foo1458::new(),
            foo1459: Foo1459::new(),
            foo1460: Foo1460::new(),
            foo1461: Foo1461::new(),
            foo1462: Foo1462::new(),
            foo1463: Foo1463::new(),
            foo1464: Foo1464::new(),
            foo1465: Foo1465::new(),
            foo1466: Foo1466::new(),
            foo1467: Foo1467::new(),
            foo1468: Foo1468::new(),
            foo1469: Foo1469::new(),
            foo1470: Foo1470::new(),
            foo1471: Foo1471::new(),
            foo1472: Foo1472::new(),
            foo1473: Foo1473::new(),
            foo1474: Foo1474::new(),
            foo1475: Foo1475::new(),
            foo1476: Foo1476::new(),
            foo1477: Foo1477::new(),
            foo1478: Foo1478::new(),
            foo1479: Foo1479::new(),
            foo1480: Foo1480::new(),
            foo1481: Foo1481::new(),
            foo1482: Foo1482::new(),
            foo1483: Foo1483::new(),
            foo1484: Foo1484::new(),
            foo1485: Foo1485::new(),
            foo1486: Foo1486::new(),
            foo1487: Foo1487::new(),
            foo1488: Foo1488::new(),
            foo1489: Foo1489::new(),
            foo1490: Foo1490::new(),
            foo1491: Foo1491::new(),
            foo1492: Foo1492::new(),
            foo1493: Foo1493::new(),
            foo1494: Foo1494::new(),
            foo1495: Foo1495::new(),
            foo1496: Foo1496::new(),
            foo1497: Foo1497::new(),
            foo1498: Foo1498::new(),
            foo1499: Foo1499::new(),
            foo1500: Foo1500::new(),
        }
    }
}

fn main() {}
