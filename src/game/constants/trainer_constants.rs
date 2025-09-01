macro_rules! define_trainer_enum {
    (
        $( class $gname:ident { $( $variant:ident ),* $(,)? } )+ $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TrainerClass {
            $( $gname, )*
            Unknown(u8),
        }

        impl From<u8> for TrainerClass {
            fn from(id: u8) -> Self {
                {
                    let __x = id;
                    // expand to a sequence of `if ... { return TrainerClass::... }` statements
                    define_trainer_enum!(@gclass_from_statements __x [] ; $( class $gname { $( $variant ),* } )+);
                    TrainerClass::Unknown(__x)
                }
            }
        }

        impl From<TrainerClass> for u8 {
            fn from(map: TrainerClass) -> Self {
                {
                    let __m = map;
                    // expand to a sequence of `if let TrainerClass::... = __m { return ... }`
                    define_trainer_enum!(@gclass_to_statements __m [] ; $( class $gname { $( $variant ),* } )+);
                    match __m {
                        TrainerClass::Unknown(x) => x,
                        _ => unreachable!("all named variants handled by generated code"),
                    }
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Trainer {
            $( $( $variant, )* )+
            Unknown(u8, u8),
        }

        impl From<(u8, u8)> for Trainer {
            fn from(id: (u8, u8)) -> Self {
                {
                    let (__x, __y) = id;
                    // expand to a sequence of `if ... { return Trainer::... }` statements
                    define_trainer_enum!(@emit_from_statements __x __y [] ; $( class $gname { $( $variant ),* } )+);
                    Trainer::Unknown(__x, __y)
                }
            }
        }

        impl From<Trainer> for (u8, u8) {
            fn from(map: Trainer) -> Self {
                {
                    let __m = map;
                    // expand to a sequence of `if let Trainer::... = __m { return (...) }`
                    define_trainer_enum!(@emit_to_statements __m [] ; $( class $gname { $( $variant ),* } )+);
                    match __m {
                        Trainer::Unknown(x, y) => (x, y),
                        _ => unreachable!("all named variants handled by generated code"),
                    }
                }
            }
        }
    };

    // ===== u8 -> TrainerClass =====
    (@gclass_from_statements $x:ident [$($gacc:tt)*] ; ) => {};
    (@gclass_from_statements $x:ident [$($gacc:tt)*] ;
        class $gname:ident { $( $v:ident ),* $(,)? } $($rest:tt)*
    ) => {
        if ($x as usize) == (define_trainer_enum!(@n $($gacc)*)) {
            return TrainerClass::$gname;
        }
        define_trainer_enum!(@gclass_from_statements $x [$($gacc)* _] ; $($rest)*);
    };

    // ===== TrainerClass -> u8 =====
    (@gclass_to_statements $m:ident [$($gacc:tt)*] ; ) => {};
    (@gclass_to_statements $m:ident [$($gacc:tt)*] ;
        class $gname:ident { $( $v:ident ),* $(,)? } $($rest:tt)*
    ) => {
        if let TrainerClass::$gname = $m {
            return (define_trainer_enum!(@n $($gacc)*)) as u8;
        }
        define_trainer_enum!(@gclass_to_statements $m [$($gacc)* _] ; $($rest)*);
    };

    // ===== (u8,u8) -> Trainer =====
    (@emit_from_statements $x:ident $y:ident [$($gacc:tt)*] ; ) => {};
    (@emit_from_statements $x:ident $y:ident [$($gacc:tt)*] ;
        class $gname:ident { $( $v:ident ),* $(,)? } $($rest:tt)*
    ) => {
        define_trainer_enum!(@emit_from_items $x $y [$($gacc)*] [] $( $v ),*);
        define_trainer_enum!(@emit_from_statements $x $y [$($gacc)* _] ; $($rest)*);
    };

    (@emit_from_items $x:ident $y:ident [$($gacc:tt)*] [$($iacc:tt)*]) => {};
    (@emit_from_items $x:ident $y:ident [$($gacc:tt)*] [$($iacc:tt)*] $head:ident $(, $tail:ident )* ) => {
        if ($x as usize) == (define_trainer_enum!(@n $($gacc)*))
           && ($y as usize) == (define_trainer_enum!(@n $($iacc)*) + 1)
        {
            return Trainer::$head;
        }
        define_trainer_enum!(@emit_from_items $x $y [$($gacc)*] [$($iacc)* _] $( $tail ),*);
    };

    // ===== Trainer -> (u8,u8) =====
    (@emit_to_statements $m:ident [$($gacc:tt)*] ; ) => {};
    (@emit_to_statements $m:ident [$($gacc:tt)*] ;
        class $gname:ident { $( $v:ident ),* $(,)? } $($rest:tt)*
    ) => {
        define_trainer_enum!(@emit_to_items $m [$($gacc)*] [] $( $v ),*);
        define_trainer_enum!(@emit_to_statements $m [$($gacc)* _] ; $($rest)*);
    };

    (@emit_to_items $m:ident [$($gacc:tt)*] [$($iacc:tt)*] $head:ident $(, $tail:ident )* ) => {
        if let Trainer::$head = $m {
            return (
                (define_trainer_enum!(@n $($gacc)*)) as u8,
                (define_trainer_enum!(@n $($iacc)*) + 1) as u8,
            );
        }
        define_trainer_enum!(@emit_to_items $m [$($gacc)*] [$($iacc)* _] $( $tail ),*);
    };
    (@emit_to_items $m:ident [$($gacc:tt)*] [$($iacc:tt)*]) => {};

    // ===== tiny counter: `_ _ _` -> N =====
    (@n) => { 0usize };
    (@n $_head:tt $($rest:tt)*) => { 1usize + define_trainer_enum!(@n $($rest)*) };
}

impl Trainer {
    pub fn class(self) -> TrainerClass {
        <(u8, u8)>::from(self).0.into()
    }
}

define_trainer_enum! {
    class TrainerNone {
        // PhonecontactMom,
        // PhonecontactBikeshop,
        // PhonecontactBill,
        // PhonecontactElm,
        // PhonecontactBuena,
    }

    class Falkner {
        Falkner1,
    }

    class Whitney {
        Whitney1,
    }

    class Bugsy {
        Bugsy1,
    }

    class Morty {
        Morty1,
    }

    class Pryce {
        Pryce1,
    }

    class Jasmine {
        Jasmine1,
    }

    class Chuck {
        Chuck1,
    }

    class Clair {
        Clair1,
    }

    class Rival1 {
        Rival11Chikorita,
        Rival11Cyndaquil,
        Rival11Totodile,
        Rival12Chikorita,
        Rival12Cyndaquil,
        Rival12Totodile,
        Rival13Chikorita,
        Rival13Cyndaquil,
        Rival13Totodile,
        Rival14Chikorita,
        Rival14Cyndaquil,
        Rival14Totodile,
        Rival15Chikorita,
        Rival15Cyndaquil,
        Rival15Totodile,
    }

    class PokemonProf {}

    class Will {
        Will1,
    }

    class Cal {
        Cal1, // unused
        Cal2,
        Cal3,
    }

    class Bruno {
        Bruno1,
    }

    class Karen {
        Karen1,
    }

    class Koga {
        Koga1,
    }

    class Champion {
        Lance,
    }

    class Brock {
        Brock1,
    }

    class Misty {
        Misty1,
    }

    class LtSurge {
        LtSurge1,
    }

    class Scientist {
        Ross,
        Mitch,
        Jed,
        Marc,
        Rich,
    }

    class Erika {
        Erika1,
    }

    class Youngster {
        Joey1,
        Mikey,
        Albert,
        Gordon,
        Samuel,
        Ian,
        Joey2,
        Joey3,
        Warren,
        Jimmy,
        Owen,
        Jason,
        Joey4,
        Joey5,
    }

    class Schoolboy {
        Jack1,
        Kipp,
        Alan1,
        Johnny,
        Danny,
        Tommy,
        Dudley,
        Joe,
        Billy,
        Chad1,
        Nate,
        Ricky,
        Jack2,
        Jack3,
        Alan2,
        Alan3,
        Chad2,
        Chad3,
        Jack4,
        Jack5,
        Alan4,
        Alan5,
        Chad4,
        Chad5,
    }

    class BirdKeeper {
        Rod,
        Abe,
        Bryan,
        Theo,
        Toby,
        Denis,
        Vance1,
        Hank,
        Roy,
        Boris,
        Bob,
        Jose1,
        Peter,
        Jose2,
        Perry,
        Bret,
        Jose3,
        Vance2,
        Vance3,
    }

    class Lass {
        Carrie,
        Bridget,
        Alice,
        Krise,
        Connie1,
        Linda,
        Laura,
        Shannon,
        Michelle,
        Dana1,
        Ellen,
        Connie2, // unused
        Connie3, // unused
        Dana2,
        Dana3,
        Dana4,
        Dana5,
    }

    class Janine {
        Janine1,
    }

    class CooltrainerM {
        Nick,
        Aaron,
        Paul,
        Cody,
        Mike,
        Gaven1,
        Gaven2,
        Ryan,
        Jake,
        Gaven3,
        Blake,
        Brian,
        Erick, // unused
        Andy, // unused
        Tyler, // unused
        Sean,
        Kevin,
        Steve, // unused
        Allen,
        Darin,
    }

    class CooltrainerF {
        Gwen,
        Lois,
        Fran,
        Lola,
        Kate,
        Irene,
        Kelly,
        Joyce,
        Beth1,
        Reena1,
        Megan,
        Beth2,
        Carol,
        Quinn,
        Emma,
        Cybil,
        Jenn,
        Beth3,
        Reena2,
        Reena3,
        Cara,
    }

    class Beauty {
        Victoria,
        Samantha,
        Julie, // unused
        Jaclyn, // unused
        Brenda, // unused
        Cassie,
        Caroline, // unused
        Carlene, // unused
        Jessica, // unused
        Rachael, // unused
        Angelica, // unused
        Kendra, // unused
        Veronica, // unused
        Julia,
        Theresa, // unused
        Valerie,
        Olivia,
    }

    class Pokemaniac {
        Larry,
        Andrew,
        Calvin,
        Shane,
        Ben,
        Brent1,
        Ron,
        Ethan,
        Brent2,
        Brent3,
        Issac,
        Donald,
        Zach,
        Brent4,
        Miller,
    }

    class GruntM {
        GruntM1,
        GruntM2,
        GruntM3,
        GruntM4,
        GruntM5,
        GruntM6,
        GruntM7,
        GruntM8,
        GruntM9,
        GruntM10,
        GruntM11,
        GruntM12, // unused
        GruntM13,
        GruntM14,
        GruntM15,
        GruntM16,
        GruntM17,
        GruntM18,
        GruntM19,
        GruntM20,
        GruntM21,
        GruntM22, // unused
        GruntM23, // unused
        GruntM24,
        GruntM25,
        GruntM26, // unused
        GruntM27, // unused
        GruntM28,
        GruntM29,
        GruntM30, // unused
        GruntM31,
    }

    class Gentleman {
        Preston,
        Edward,
        Gregory,
        Virgil, // unused
        Alfred,
    }

    class Skier {
        Roxanne,
        Clarissa,
    }

    class Teacher {
        Colette,
        Hillary,
        Shirley,
    }

    class Sabrina {
        Sabrina1,
    }

    class BugCatcher {
        Don,
        Rob,
        Ed,
        Wade1,
        BugCatcherBenny,
        Al,
        Josh,
        Arnie1,
        Ken,
        Wade2,
        Wade3,
        Doug,
        Arnie2,
        Arnie3,
        Wade4,
        Wade5,
        Arnie4,
        Arnie5,
        Wayne,
    }

    class Fisher {
        Justin,
        Ralph1,
        Arnold,
        Kyle,
        Henry,
        Marvin,
        Tully1,
        Andre,
        Raymond,
        Wilton1,
        Edgar,
        Jonah,
        Martin,
        Stephen,
        Barney,
        Ralph2,
        Ralph3,
        Tully2,
        Tully3,
        Wilton2,
        Scott,
        Wilton3,
        Ralph4,
        Ralph5,
        Tully4,
    }

    class SwimmerM {
        Harold,
        Simon,
        Randall,
        Charlie,
        George,
        Berke,
        Kirk,
        Mathew,
        Hal, // unused
        Paton, // unused
        Daryl, // unused
        Walter, // unused
        Tony, // unused
        Jerome,
        Tucker,
        Rick, // unused
        Cameron,
        Seth,
        James, // unused
        Lewis, // unused
        Parker,
    }

    class SwimmerF {
        Elaine,
        Paula,
        Kaylee,
        Susie,
        Denise,
        Kara,
        Wendy,
        Lisa, // unused
        Jill, // unused
        Mary, // unused
        Katie, // unused
        Dawn,
        Tara, // unused
        Nicole,
        Lori,
        Jody, // unused
        Nikki,
        Diana,
        Briana,
    }

    class Sailor {
        Eugene,
        Huey1,
        Terrell,
        Kent,
        Ernest,
        Jeff,
        Garrett,
        Kenneth,
        Stanly,
        Harry,
        Huey2,
        Huey3,
        Huey4,
    }

    class SuperNerd {
        Stan,
        Eric,
        Gregg, // unused
        Jay, // unused
        Dave, // unused
        Sam,
        Tom,
        Pat,
        Shawn,
        Teru,
        Russ, // unused
        Norton, // unused
        Hugh,
        Markus,
    }

    class Rival2 {
        Rival21Chikorita,
        Rival21Cyndaquil,
        Rival21Totodile,
        Rival22Chikorita,
        Rival22Cyndaquil,
        Rival22Totodile,
    }

    class Guitarist {
        Clyde,
        Vincent,
    }

    class Hiker {
        Anthony1,
        Russell,
        Phillip,
        Leonard,
        Anthony2,
        Benjamin,
        Erik,
        Michael,
        Parry1,
        Timothy,
        Bailey,
        Anthony3,
        Tim,
        Noland,
        Sidney,
        Kenny,
        Jim,
        Daniel,
        Parry2,
        Parry3,
        Anthony4,
        Anthony5,
    }

    class Biker {
        BikerBenny, // unused
        Kazu, // unused
        Dwayne,
        Harris,
        Zeke,
        Charles,
        Riley,
        Joel,
        Glenn,
    }

    class Blaine {
        Blaine1,
    }

    class Burglar {
        Duncan,
        Eddie,
        Corey,
    }

    class Firebreather {
        Otis,
        Dick, // unused
        Ned, // unused
        Burt,
        Bill,
        Walt,
        Ray,
        Lyle,
    }

    class Juggler {
        Irwin1,
        Fritz,
        Horton,
        Irwin2, // unused
        Irwin3, // unused
        Irwin4, // unused
    }

    class Blackbelt {
        Kenji1, // unused
        Yoshi,
        Kenji2, // unused
        Lao,
        Nob,
        Kiyo,
        Lung,
        Kenji3,
        Wai,
    }

    class ExecutiveM {
        ExecutiveM1,
        ExecutiveM2,
        ExecutiveM3,
        ExecutiveM4,
    }

    class Psychic {
        Nathan,
        Franklin,
        Herman,
        Fidel,
        Greg,
        Norman,
        Mark,
        Phil,
        Richard,
        Gilbert,
        Jared,
        Rodney,
    }

    class Picnicker {
        Liz1,
        Gina1,
        Brooke,
        Kim,
        Cindy,
        Hope,
        Sharon,
        Debra,
        Gina2,
        Erin1,
        Liz2,
        Liz3,
        Heidi,
        Edna,
        Gina3,
        Tiffany1,
        Tiffany2,
        Erin2,
        Tanya,
        Tiffany3,
        Erin3,
        Liz4,
        Liz5,
        Gina4,
        Gina5,
        Tiffany4,
    }

    class Camper {
        Roland,
        Todd1,
        Ivan,
        Elliot,
        Barry,
        Lloyd,
        Dean,
        Sid,
        Harvey, // unused
        Dale, // unused
        Ted,
        Todd2,
        Todd3,
        Thomas, // unused
        Leroy, // unused
        David, // unused
        John, // unused
        Jerry,
        Spencer,
        Todd4,
        Todd5,
        Quentin,
    }

    class ExecutiveF {
        ExecutiveF1,
        ExecutiveF2,
    }

    class Sage {
        Chow,
        Nico,
        Jin,
        Troy,
        Jeffrey,
        Ping,
        Edmond,
        Neal,
        Li,
        Gaku,
        Masa,
        Koji,
    }

    class Medium {
        Martha,
        Grace,
        Bethany, // unused
        Margret, // unused
        Ethel, // unused
        Rebecca,
        Doris,
    }

    class Boarder {
        Ronald,
        Brad,
        Douglas,
    }

    class PokefanM {
        William,
        Derek1,
        Robert,
        Joshua,
        Carter,
        Trevor,
        Brandon,
        Jeremy,
        Colin,
        Derek2, // unused
        Derek3, // unused
        Alex,
        Rex,
        Allan,
    }

    class KimonoGirl {
        NaokoUnused, // unused
        Naoko,
        Sayo,
        Zuki,
        Kuni,
        Miki,
    }

    class Twins {
        AmyAndMay1,
        AnnAndAnne1,
        AnnAndAnne2,
        AmyAndMay2,
        JoAndZoe1,
        JoAndZoe2,
        MegAndPeg1,
        MegAndPeg2,
        LeaAndPia1,
        LeaAndPia2, // unused
    }

    class PokefanF {
        Beverly1,
        Ruth,
        Beverly2, // unused
        Beverly3, // unused
        Georgia,
        Jaime,
    }

    class Red {
        Red1,
    }

    class Blue {
        Blue1,
    }

    class Officer {
        Keith,
        Dirk,
    }

    class GruntF {
        GruntF1,
        GruntF2,
        GruntF3,
        GruntF4,
        GruntF5,
    }

    class Mysticalman {
        Eusine,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_enum() {
        assert_eq!((0x35, 19), Trainer::Tanya.into());
        assert_eq!(Trainer::Tanya, Trainer::from((0x35, 19)));

        assert_eq!((0x43, 1), Trainer::Eusine.into());
        assert_eq!(Trainer::Eusine, Trainer::from((0x43, 1)));
    }
}
