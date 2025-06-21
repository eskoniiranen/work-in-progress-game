use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlotItem {
    Empty,
    Item(Item),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OffHandSlot {
    Empty,
    OffHand(Item),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub head: SlotItem,
    pub body: SlotItem,
    pub hands: SlotItem,
    pub legs: SlotItem,
    pub feet: SlotItem,
    pub weapon: SlotItem,
    pub off_hand: OffHandSlot,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            head: SlotItem::Empty,
            body: SlotItem::Empty,
            hands: SlotItem::Empty,
            legs: SlotItem::Empty,
            feet: SlotItem::Empty,
            weapon: SlotItem::Empty,
            off_hand: OffHandSlot::Empty,
        }
    }
}

use crate::character::item::{Item, ItemKind};

impl Inventory {
    pub fn equip_item(&mut self, item: Item) -> Result<(), String> {
        match &item.kind {
            ItemKind::MeleeWeapon(_) | ItemKind::RangedWeapon(_) | ItemKind::MagicalWeapon(_) => {
                self.weapon = SlotItem::Item(item);
                Ok(())
            }
            ItemKind::Offhand(_) => {
                self.off_hand = OffHandSlot::OffHand(item);
                Ok(())
            }
        }
    }

    pub fn unequip_weapon(&mut self) -> Option<Item> {
        match std::mem::replace(&mut self.weapon, SlotItem::Empty) {
            SlotItem::Item(item) => Some(item),
            _ => None,
        }
    }

    pub fn unequip_offhand(&mut self) -> Option<Item> {
        match std::mem::replace(&mut self.off_hand, OffHandSlot::Empty) {
            OffHandSlot::OffHand(item) => Some(item),
            _ => None,
        }
    }
}
