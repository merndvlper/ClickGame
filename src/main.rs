mod egg;
mod hero;
mod upgrade;

use macroquad::prelude::*;
use crate::hero::Hero;
use crate::upgrade::{Upgrade};

#[macroquad::main("Clicker Game")]
async fn main() {
    let (x, y) = (screen_width() , screen_height() / 2.+50.);
    let r = 140.;
    let circle = Circle::new(x, y, r);

    let (x2, y2) = (screen_width()-128. , screen_height() / 2.+293.);
    let rectangle = Rect::new(x2,y2,250.,64.);

    let mut counter = 0;

    let mut upgrade = Upgrade{
        damage_lvl:1,
        cost:100.0,
        hp: 100.0,
        lvl: 1,
        reward: 100.0,
    };


    let mut hero = Hero {
        damage: 10.0,
        point: 0.0,
    };

    let mut new_hp = upgrade.hp;

    loop {
        clear_background(LIGHTGRAY);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mouse_rect = Rect::new(mouse_x, mouse_y, 1.,1.);
            let mouse_circ = Circle::new(mouse_x, mouse_y, 1.);


            if rectangle.overlaps(&mouse_rect){
                if hero.point >= upgrade.cost{
                    if counter==0 {
                        hero.damage += 10.0;
                        Upgrade::update_damage_lvl(upgrade.damage_lvl,upgrade.cost);
                        hero.point-=upgrade.cost;
                        counter+=1;
                    }else if counter==1 {
                        hero.damage +=10.0;
                        Upgrade::update_damage_lvl(upgrade.damage_lvl,upgrade.cost);
                        upgrade.cost=Upgrade::update_cost(upgrade.cost);
                        hero.point-=upgrade.cost;
                    }
                }
            }

            if circle.overlaps(&mouse_circ) {
                if upgrade.hp >= 1. {
                    upgrade.hp -= hero.damage;
                }else if upgrade.hp<=0. {
                    new_hp = Upgrade::hp_update(new_hp);
                    upgrade.reward = Upgrade::reward_update(upgrade.reward);
                    hero.point += upgrade.reward;
                    upgrade.lvl = Upgrade::lvl_update(upgrade.lvl);
                    upgrade.hp=new_hp;
                }
            }
        }


        draw_text(
            format!("Boss HP: {} \n Boss LVL:{}",upgrade.hp,upgrade.lvl).as_str(),
            screen_width()/2.-250.,
            150.,
            50.,
            DARKBLUE
        );
        draw_text(
            format!("Hero Point {}",hero.point).as_str(),
            screen_width()/2.+100.,
            550.,
            50.,
            DARKBLUE
        );

        draw_circle(x,y,r,VIOLET);
        draw_rectangle(x2, y2, 250., 64., DARKBLUE);
        draw_text(
            format!("Upgrade! {} Point",upgrade.cost).as_str(),
            screen_width()/2.-50. ,
            screen_height() / 2.+234.,
            22.5,
            GREEN
        );

        draw_text(
            format!("Hero Power {}",hero.damage).as_str(),
            screen_width()/2.-300.,
            550.,
            50.,
            DARKBLUE
        );
        next_frame().await;
    }
}