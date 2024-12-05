mod memory;

use crate::memory::enums::BufType;
use crate::memory::handler::MemoryHandler;

struct PlayerOffsets {
    coins: u64,
    badguys: u64,
}

const PLAYEROFFSETS: PlayerOffsets = PlayerOffsets {
    coins: 0xF0,
    badguys: 0xF4,
};

fn main() {
    let mut handle = MemoryHandler::new();

    if !handle.init("supertux2", "rx-p") {
        return;
    }

    let pid = handle.pid.expect("Failed setting pid.");
    let base_addr = handle
        .base_addr
        .expect("Failed fetching for the base address.");

    println!("proc: {}:{:x}", pid, base_addr);

    let local_player_addr = handle.resolve_ptr_chain(pid, base_addr, &[0xC70, 0x10], &BufType::U64);
    let coins_addr = handle.resolve_ptr_chain(pid, base_addr, &[0xA40, 0x8, 0x20], &BufType::U64);
    let cords_addr =
        handle.resolve_ptr_chain(pid, base_addr, &[0xE38, 0x50, 0x0, 0x110], &BufType::U64) + 0x30;
    println!("lplr: {:x}", local_player_addr);
    println!("coins: {:x}", coins_addr);
    println!("cords: {:x}", cords_addr);

    let plr_coins_addr = local_player_addr + PLAYEROFFSETS.coins;
    let plr_badguys_addr = local_player_addr + PLAYEROFFSETS.badguys;

    let pcoins = handle.read_addr(pid, coins_addr, &BufType::U32).unwrap();

    let xcords = handle.read_addr(pid, cords_addr, &BufType::Float).unwrap();
    let ycords = handle
        .read_addr(pid, cords_addr + 0x4, &BufType::Float)
        .unwrap();

    let lcoins = handle
        .read_addr(pid, plr_coins_addr, &BufType::U32)
        .unwrap();
    let lbadguys = handle
        .read_addr(pid, plr_badguys_addr, &BufType::U32)
        .unwrap();

    println!("profile: coins={}", pcoins);
    println!("cords: x={} y={}", xcords, ycords);
    println!("stage: coins={} badguys={}", lcoins, lbadguys);
}
