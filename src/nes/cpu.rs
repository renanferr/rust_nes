use super::bus::Bus;

const CPU6502_LOOKUP: [INSTRUCTION; 256] = [
    INSTRUCTION{ name: "BRK", operate: CPU6502::BRK, addrmode: CPU6502::IMM, cycles: 7 },INSTRUCTION{ name: "ORA", operate: CPU6502::ORA, addrmode: CPU6502::IZX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 3 },INSTRUCTION{ name: "ORA", operate: CPU6502::ORA, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "ASL", operate: CPU6502::ASL, addrmode: CPU6502::ZP0, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "PHP", operate: CPU6502::PHP, addrmode: CPU6502::IMP, cycles: 3 },INSTRUCTION{ name: "ORA", operate: CPU6502::ORA, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "ASL", operate: CPU6502::ASL, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "ORA", operate: CPU6502::ORA, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "ASL", operate: CPU6502::ASL, addrmode: CPU6502::ABS, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },
    INSTRUCTION{ name: "BPL", operate: CPU6502::BPL, addrmode: CPU6502::REL, cycles: 2 },INSTRUCTION{ name: "ORA", operate: CPU6502::ORA, addrmode: CPU6502::IZY, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "ORA", operate: CPU6502::ORA, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "ASL", operate: CPU6502::ASL, addrmode: CPU6502::ZPX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "CLC", operate: CPU6502::CLC, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "ORA", operate: CPU6502::ORA, addrmode: CPU6502::ABY, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "ORA", operate: CPU6502::ORA, addrmode: CPU6502::ABX, cycles: 4 },INSTRUCTION{ name: "ASL", operate: CPU6502::ASL, addrmode: CPU6502::ABX, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },
    INSTRUCTION{ name: "JSR", operate: CPU6502::JSR, addrmode: CPU6502::ABS, cycles: 6 },INSTRUCTION{ name: "AND", operate: CPU6502::AND, addrmode: CPU6502::IZX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "BIT", operate: CPU6502::BIT, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "AND", operate: CPU6502::AND, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "ROL", operate: CPU6502::ROL, addrmode: CPU6502::ZP0, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "PLP", operate: CPU6502::PLP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "AND", operate: CPU6502::AND, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "ROL", operate: CPU6502::ROL, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "BIT", operate: CPU6502::BIT, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "AND", operate: CPU6502::AND, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "ROL", operate: CPU6502::ROL, addrmode: CPU6502::ABS, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },
    INSTRUCTION{ name: "BMI", operate: CPU6502::BMI, addrmode: CPU6502::REL, cycles: 2 },INSTRUCTION{ name: "AND", operate: CPU6502::AND, addrmode: CPU6502::IZY, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "AND", operate: CPU6502::AND, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "ROL", operate: CPU6502::ROL, addrmode: CPU6502::ZPX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "SEC", operate: CPU6502::SEC, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "AND", operate: CPU6502::AND, addrmode: CPU6502::ABY, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "AND", operate: CPU6502::AND, addrmode: CPU6502::ABX, cycles: 4 },INSTRUCTION{ name: "ROL", operate: CPU6502::ROL, addrmode: CPU6502::ABX, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },
    INSTRUCTION{ name: "RTI", operate: CPU6502::RTI, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "EOR", operate: CPU6502::EOR, addrmode: CPU6502::IZX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 3 },INSTRUCTION{ name: "EOR", operate: CPU6502::EOR, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "LSR", operate: CPU6502::LSR, addrmode: CPU6502::ZP0, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "PHA", operate: CPU6502::PHA, addrmode: CPU6502::IMP, cycles: 3 },INSTRUCTION{ name: "EOR", operate: CPU6502::EOR, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "LSR", operate: CPU6502::LSR, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "JMP", operate: CPU6502::JMP, addrmode: CPU6502::ABS, cycles: 3 },INSTRUCTION{ name: "EOR", operate: CPU6502::EOR, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "LSR", operate: CPU6502::LSR, addrmode: CPU6502::ABS, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },
    INSTRUCTION{ name: "BVC", operate: CPU6502::BVC, addrmode: CPU6502::REL, cycles: 2 },INSTRUCTION{ name: "EOR", operate: CPU6502::EOR, addrmode: CPU6502::IZY, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "EOR", operate: CPU6502::EOR, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "LSR", operate: CPU6502::LSR, addrmode: CPU6502::ZPX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "CLI", operate: CPU6502::CLI, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "EOR", operate: CPU6502::EOR, addrmode: CPU6502::ABY, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "EOR", operate: CPU6502::EOR, addrmode: CPU6502::ABX, cycles: 4 },INSTRUCTION{ name: "LSR", operate: CPU6502::LSR, addrmode: CPU6502::ABX, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },
    INSTRUCTION{ name: "RTS", operate: CPU6502::RTS, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "ADC", operate: CPU6502::ADC, addrmode: CPU6502::IZX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 3 },INSTRUCTION{ name: "ADC", operate: CPU6502::ADC, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "ROR", operate: CPU6502::ROR, addrmode: CPU6502::ZP0, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "PLA", operate: CPU6502::PLA, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "ADC", operate: CPU6502::ADC, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "ROR", operate: CPU6502::ROR, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "JMP", operate: CPU6502::JMP, addrmode: CPU6502::IND, cycles: 5 },INSTRUCTION{ name: "ADC", operate: CPU6502::ADC, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "ROR", operate: CPU6502::ROR, addrmode: CPU6502::ABS, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },
    INSTRUCTION{ name: "BVS", operate: CPU6502::BVS, addrmode: CPU6502::REL, cycles: 2 },INSTRUCTION{ name: "ADC", operate: CPU6502::ADC, addrmode: CPU6502::IZY, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "ADC", operate: CPU6502::ADC, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "ROR", operate: CPU6502::ROR, addrmode: CPU6502::ZPX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "SEI", operate: CPU6502::SEI, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "ADC", operate: CPU6502::ADC, addrmode: CPU6502::ABY, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "ADC", operate: CPU6502::ADC, addrmode: CPU6502::ABX, cycles: 4 },INSTRUCTION{ name: "ROR", operate: CPU6502::ROR, addrmode: CPU6502::ABX, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },
    INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "STA", operate: CPU6502::STA, addrmode: CPU6502::IZX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "STY", operate: CPU6502::STY, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "STA", operate: CPU6502::STA, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "STX", operate: CPU6502::STX, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 3 },INSTRUCTION{ name: "DEY", operate: CPU6502::DEY, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "TXA", operate: CPU6502::TXA, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "STY", operate: CPU6502::STY, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "STA", operate: CPU6502::STA, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "STX", operate: CPU6502::STX, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 4 },
    INSTRUCTION{ name: "BCC", operate: CPU6502::BCC, addrmode: CPU6502::REL, cycles: 2 },INSTRUCTION{ name: "STA", operate: CPU6502::STA, addrmode: CPU6502::IZY, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "STY", operate: CPU6502::STY, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "STA", operate: CPU6502::STA, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "STX", operate: CPU6502::STX, addrmode: CPU6502::ZPY, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "TYA", operate: CPU6502::TYA, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "STA", operate: CPU6502::STA, addrmode: CPU6502::ABY, cycles: 5 },INSTRUCTION{ name: "TXS", operate: CPU6502::TXS, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "STA", operate: CPU6502::STA, addrmode: CPU6502::ABX, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },
    INSTRUCTION{ name: "LDY", operate: CPU6502::LDY, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "LDA", operate: CPU6502::LDA, addrmode: CPU6502::IZX, cycles: 6 },INSTRUCTION{ name: "LDX", operate: CPU6502::LDX, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "LDY", operate: CPU6502::LDY, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "LDA", operate: CPU6502::LDA, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "LDX", operate: CPU6502::LDX, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 3 },INSTRUCTION{ name: "TAY", operate: CPU6502::TAY, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "LDA", operate: CPU6502::LDA, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "TAX", operate: CPU6502::TAX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "LDY", operate: CPU6502::LDY, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "LDA", operate: CPU6502::LDA, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "LDX", operate: CPU6502::LDX, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 4 },
    INSTRUCTION{ name: "BCS", operate: CPU6502::BCS, addrmode: CPU6502::REL, cycles: 2 },INSTRUCTION{ name: "LDA", operate: CPU6502::LDA, addrmode: CPU6502::IZY, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "LDY", operate: CPU6502::LDY, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "LDA", operate: CPU6502::LDA, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "LDX", operate: CPU6502::LDX, addrmode: CPU6502::ZPY, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "CLV", operate: CPU6502::CLV, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "LDA", operate: CPU6502::LDA, addrmode: CPU6502::ABY, cycles: 4 },INSTRUCTION{ name: "TSX", operate: CPU6502::TSX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "LDY", operate: CPU6502::LDY, addrmode: CPU6502::ABX, cycles: 4 },INSTRUCTION{ name: "LDA", operate: CPU6502::LDA, addrmode: CPU6502::ABX, cycles: 4 },INSTRUCTION{ name: "LDX", operate: CPU6502::LDX, addrmode: CPU6502::ABY, cycles: 4 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 4 },
    INSTRUCTION{ name: "CPY", operate: CPU6502::CPY, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "CMP", operate: CPU6502::CMP, addrmode: CPU6502::IZX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "CPY", operate: CPU6502::CPY, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "CMP", operate: CPU6502::CMP, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "DEC", operate: CPU6502::DEC, addrmode: CPU6502::ZP0, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "INY", operate: CPU6502::INY, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "CMP", operate: CPU6502::CMP, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "DEX", operate: CPU6502::DEX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "CPY", operate: CPU6502::CPY, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "CMP", operate: CPU6502::CMP, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "DEC", operate: CPU6502::DEC, addrmode: CPU6502::ABS, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },
    INSTRUCTION{ name: "BNE", operate: CPU6502::BNE, addrmode: CPU6502::REL, cycles: 2 },INSTRUCTION{ name: "CMP", operate: CPU6502::CMP, addrmode: CPU6502::IZY, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "CMP", operate: CPU6502::CMP, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "DEC", operate: CPU6502::DEC, addrmode: CPU6502::ZPX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "CLD", operate: CPU6502::CLD, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "CMP", operate: CPU6502::CMP, addrmode: CPU6502::ABY, cycles: 4 },INSTRUCTION{ name: "NOP", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "CMP", operate: CPU6502::CMP, addrmode: CPU6502::ABX, cycles: 4 },INSTRUCTION{ name: "DEC", operate: CPU6502::DEC, addrmode: CPU6502::ABX, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },
    INSTRUCTION{ name: "CPX", operate: CPU6502::CPX, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "SBC", operate: CPU6502::SBC, addrmode: CPU6502::IZX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "CPX", operate: CPU6502::CPX, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "SBC", operate: CPU6502::SBC, addrmode: CPU6502::ZP0, cycles: 3 },INSTRUCTION{ name: "INC", operate: CPU6502::INC, addrmode: CPU6502::ZP0, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 5 },INSTRUCTION{ name: "INX", operate: CPU6502::INX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "SBC", operate: CPU6502::SBC, addrmode: CPU6502::IMM, cycles: 2 },INSTRUCTION{ name: "NOP", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::SBC, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "CPX", operate: CPU6502::CPX, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "SBC", operate: CPU6502::SBC, addrmode: CPU6502::ABS, cycles: 4 },INSTRUCTION{ name: "INC", operate: CPU6502::INC, addrmode: CPU6502::ABS, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },
    INSTRUCTION{ name: "BEQ", operate: CPU6502::BEQ, addrmode: CPU6502::REL, cycles: 2 },INSTRUCTION{ name: "SBC", operate: CPU6502::SBC, addrmode: CPU6502::IZY, cycles: 5 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 8 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "SBC", operate: CPU6502::SBC, addrmode: CPU6502::ZPX, cycles: 4 },INSTRUCTION{ name: "INC", operate: CPU6502::INC, addrmode: CPU6502::ZPX, cycles: 6 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 6 },INSTRUCTION{ name: "SED", operate: CPU6502::SED, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "SBC", operate: CPU6502::SBC, addrmode: CPU6502::ABY, cycles: 4 },INSTRUCTION{ name: "NOP", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 2 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::NOP, addrmode: CPU6502::IMP, cycles: 4 },INSTRUCTION{ name: "SBC", operate: CPU6502::SBC, addrmode: CPU6502::ABX, cycles: 4 },INSTRUCTION{ name: "INC", operate: CPU6502::INC, addrmode: CPU6502::ABX, cycles: 7 },INSTRUCTION{ name: "???", operate: CPU6502::XXX, addrmode: CPU6502::IMP, cycles: 7 },
];

trait Flags {
    const C: u8;
    const Z: u8;
    const I: u8;
    const D: u8;
    const B: u8;
    const U: u8;
    const V: u8;
    const N: u8;
}

struct Flags6502;

impl Flags for Flags6502 {
    const C: u8 = (1 << 0); // carry bit
    const Z: u8 = (1 << 1); // zero
    const I: u8 = (1 << 2); // disable interrupts
    const D: u8 = (1 << 3); // decimal mode
    const B: u8 = (1 << 4); // break
    const U: u8 = (1 << 5); // unused
    const V: u8 = (1 << 6); // overflow
    const N: u8 = (1 << 7); // negative
}

struct INSTRUCTION {
    name: &'static str,
    operate: fn() -> u8,
    addrmode: fn() -> u8,
    cycles: u8,
    // C++ code for 6502 instruction struct:
    // std::string name
    // uint8_t     (CPU6502::*operate )(void) = nullptr;
    // uint8_t     (CPU6502::*addrmode)(void) = nullptr;
    // uint8_t     cycles = 0;
}

pub struct CPU6502 {
    a: u8,      // accumulator register
    x: u8,      // x register
    y: u8,      // y register
    stkp: u8,   // stack pointer
    pc: u16,    // program counter
    status: u8, // status register

    fetched: u8,
    temp: u16,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    clock_count: u32,

    bus: Bus,

    lookup: [INSTRUCTION; 256],
    // logfile:            std::fs::File,
}

impl CPU6502 {
    pub fn new(b: Bus) -> CPU6502 {
        return CPU6502 {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            pc: 0x0000,
            status: 0x00,

            fetched: 0x00,
            temp: 0x0000,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0x00,
            clock_count: 0x000000,

            bus: b,

            lookup: CPU6502_LOOKUP,
        };
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.bus.read(addr, false)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        println!("writing {:?} to {:?}", data, addr);
        self.bus.write(addr, data)
    }

    pub fn connect_bus(&mut self, b: Bus) {
        self.bus = b;
    }

    pub fn reset(&self) {
        // Get address to set program counter to
        self.addr_abs = 0xFFFC;
        let lo = self.read(self.addr_abs + 0);
        let hi = self.read(self.addr_abs + 1);

        // Set it
        self.pc = ((hi << 8) | lo).into();

        // Reset internal registers
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00 | Flags6502::U;

        // Clear internal helper variables
        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        // Reset takes time
        self.cycles = 8;
    }
}

