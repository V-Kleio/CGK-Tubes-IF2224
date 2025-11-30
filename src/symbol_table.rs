use crate::types::{DataType, ObjectKind};
use std::fmt;

// This uses Backward chaining

/// Entry in the identifier table (tab)
#[derive(Debug, Clone)]
pub struct TabEntry {
    pub name: String,
    pub link: Option<usize>,      // Pointer to previous identifier in same scope
    pub obj: ObjectKind,           // Kind of object
    pub data_type: DataType,       // Type of the identifier
    pub ref_index: Option<usize>,  // Reference to atab/btab for composite types
    pub normal: bool,              // true = normal variable, false = var parameter
    pub level: usize,              // Lexical level
    pub address: usize,            // Offset/value depending on obj type
}

/// Entry in the block table (btab)
#[derive(Debug, Clone)]
pub struct BTabEntry {
    pub last: usize,      // Last identifier in this block
    pub last_par: usize,  // Last parameter
    pub param_size: usize, // Total parameter size
    pub var_size: usize,  // Total local variable size
}

/// Entry in the array table (atab)
#[derive(Debug, Clone)]
pub struct ATabEntry {
    pub index_type: DataType,      // Type of array index
    pub element_type: DataType,    // Type of array elements
    pub element_ref: Option<usize>, // Reference if element is composite
    pub low_bound: i32,            // Lower bound of array
    pub high_bound: i32,           // Upper bound of array
    pub element_size: usize,       // Size of one element
    pub total_size: usize,         // Total size of array
}

/// Symbol table with three tables: tab, btab, atab
pub struct SymbolTable {
    pub tab: Vec<TabEntry>,
    pub btab: Vec<BTabEntry>,
    pub atab: Vec<ATabEntry>,
    pub display: Vec<usize>, // Display stack for scope management
}

impl SymbolTable {
    /// Create a new symbol table initialized with reserved words and predefined identifiers
    pub fn new() -> Self {
        let mut tab = Vec::new();
        
        // ============================================================
        // RESERVED WORDS (indices 0-28) - 29 entries
        // ============================================================
        
        // 0: AND (dan)
        tab.push(TabEntry {
            name: "dan".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 0,
        });
        
        // 1: ARRAY (larik)
        tab.push(TabEntry {
            name: "larik".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 1,
        });
        
        // 2: BEGIN (mulai)
        tab.push(TabEntry {
            name: "mulai".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 2,
        });
        
        // 3: CASE (kasus)
        tab.push(TabEntry {
            name: "kasus".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 3,
        });
        
        // 4: CONST (konstanta)
        tab.push(TabEntry {
            name: "konstanta".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 4,
        });
        
        // 5: DIV (bagi)
        tab.push(TabEntry {
            name: "bagi".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 5,
        });
        
        // 6: DOWNTO (turun_ke)
        tab.push(TabEntry {
            name: "turun_ke".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 6,
        });
        
        // 7: DO (lakukan)
        tab.push(TabEntry {
            name: "lakukan".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 7,
        });
        
        // 8: ELSE (selain_itu)
        tab.push(TabEntry {
            name: "selain_itu".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 8,
        });
        
        // 9: END (selesai)
        tab.push(TabEntry {
            name: "selesai".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 9,
        });
        
        // 10: FOR (untuk)
        tab.push(TabEntry {
            name: "untuk".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 10,
        });
        
        // 11: FUNCTION (fungsi)
        tab.push(TabEntry {
            name: "fungsi".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 11,
        });
        
        // 12: IF (jika)
        tab.push(TabEntry {
            name: "jika".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 12,
        });
        
        // 13: MOD (mod)
        tab.push(TabEntry {
            name: "mod".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 13,
        });
        
        // 14: NOT (tidak)
        tab.push(TabEntry {
            name: "tidak".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 14,
        });
        
        // 15: OF (dari)
        tab.push(TabEntry {
            name: "dari".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 15,
        });
        
        // 16: OR (atau)
        tab.push(TabEntry {
            name: "atau".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 16,
        });
        
        // 17: PROCEDURE (prosedur)
        tab.push(TabEntry {
            name: "prosedur".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 17,
        });
        
        // 18: PROGRAM (program)
        tab.push(TabEntry {
            name: "program".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 18,
        });
        
        // 19: RECORD (rekaman)
        tab.push(TabEntry {
            name: "rekaman".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 19,
        });
        
        // 20: REPEAT (ulangi)
        tab.push(TabEntry {
            name: "ulangi".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 20,
        });
        
        // 21: STRING (string) - Note: not in dfa_rules.json keywords
        tab.push(TabEntry {
            name: "string".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::String,
            ref_index: None,
            normal: true,
            level: 0,
            address: 21,
        });
        
        // 22: THEN (maka)
        tab.push(TabEntry {
            name: "maka".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 22,
        });
        
        // 23: TO (ke)
        tab.push(TabEntry {
            name: "ke".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 23,
        });
        
        // 24: TYPE (tipe)
        tab.push(TabEntry {
            name: "tipe".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 24,
        });
        
        // 25: UNTIL (sampai)
        tab.push(TabEntry {
            name: "sampai".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 25,
        });
        
        // 26: VAR (variabel)
        tab.push(TabEntry {
            name: "variabel".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 26,
        });
        
        // 27: WHILE (selama)
        tab.push(TabEntry {
            name: "selama".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 27,
        });
        
        // 28: PACKED (padat)
        tab.push(TabEntry {
            name: "padat".to_string(),
            link: None,
            obj: ObjectKind::Type,
            data_type: DataType::Unknown,
            ref_index: None,
            normal: true,
            level: 0,
            address: 28,
        });
        
        // ============================================================
        // PREDEFINED PROCEDURES (indices 29-32)
        // These are always available and cannot be redeclared
        // ============================================================
        
        // 29: writeln
        tab.push(TabEntry {
            name: "writeln".to_string(),
            link: None,
            obj: ObjectKind::Procedure,
            data_type: DataType::Void,
            ref_index: None,
            normal: true,
            level: 0,
            address: 29,
        });
        
        // 30: write
        tab.push(TabEntry {
            name: "write".to_string(),
            link: None,
            obj: ObjectKind::Procedure,
            data_type: DataType::Void,
            ref_index: None,
            normal: true,
            level: 0,
            address: 30,
        });
        
        // 31: readln
        tab.push(TabEntry {
            name: "readln".to_string(),
            link: None,
            obj: ObjectKind::Procedure,
            data_type: DataType::Void,
            ref_index: None,
            normal: true,
            level: 0,
            address: 31,
        });
        
        // 32: read
        tab.push(TabEntry {
            name: "read".to_string(),
            link: None,
            obj: ObjectKind::Procedure,
            data_type: DataType::Void,
            ref_index: None,
            normal: true,
            level: 0,
            address: 32,
        });
        
        // ============================================================
        // USER IDENTIFIERS START FROM INDEX 33
        // ============================================================
        
        // Initialize btab with global block (index 0)
        let btab = vec![BTabEntry {
            last: 0,
            last_par: 0,
            param_size: 0,
            var_size: 0,
        }];
        
        let atab = Vec::new();
        let display = vec![0]; // Display[0] points to global block
        
        SymbolTable {
            tab,
            btab,
            atab,
            display,
        }
    }
    
    /// Enter a new block (for procedures, functions, or main program)
    pub fn enter_block(&mut self) -> usize {
        let block_index = self.btab.len();
        self.btab.push(BTabEntry {
            last: 0,
            last_par: 0,
            param_size: 0,
            var_size: 0,
        });
        self.display.push(block_index);
        block_index
    }
    
    /// Exit current block
    pub fn exit_block(&mut self) {
        if self.display.len() > 1 {
            self.display.pop();
        }
    }
    
    /// Get current lexical level
    pub fn current_level(&self) -> usize {
        self.display.len() - 1
    }
    
    /// Insert a new identifier into the symbol table
    pub fn insert(&mut self, entry: TabEntry) -> usize {
        let index = self.tab.len();
        let level = self.current_level();
        let block_index = self.display[level];
        
        let mut entry = entry;
        
        // Find previous identifier of same object type in current block for linking
        let mut prev_same_type: Option<usize> = None;
        let mut current = self.btab[block_index].last;
        
        while current > 0 {
            if self.tab[current].obj == entry.obj {
                prev_same_type = Some(current);
                break;
            }
            current = self.tab[current].link.unwrap_or(0);
        }
        
        entry.link = prev_same_type;  // Link to previous entry of same type (or None)
        
        self.tab.push(entry);
        
        // Update btab.last to point to the most recently inserted identifier
        self.btab[block_index].last = index;
        
        index
    }
    
    /// Lookup an identifier in current and outer scopes
    pub fn lookup(&self, name: &str) -> Option<usize> {
        // Search from current level down to global level
        for level in (0..=self.current_level()).rev() {
            let block_index = self.display[level];
            let mut current = self.btab[block_index].last;  // Points to last (most recent) identifier
            
            // Follow the backward linked list in this block
            while current > 0 {
                if self.tab[current].name == name {
                    return Some(current);
                }
                current = self.tab[current].link.unwrap_or(0);
            }
        }
        
        // Check reserved words and predefined procedures (indices 0-32)
        for i in 0..33 {
            if self.tab[i].name == name {
                return Some(i);
            }
        }
        
        // Check for dynamically inserted identifiers after index 32
        for i in 33..self.tab.len() {
            if self.tab[i].name == name && self.tab[i].level == 0 {
                return Some(i);
            }
        }
        
        None
    }
    
    /// Insert new identifier at global level after user declarations have completed
    pub fn insert_at_global(&mut self, mut entry: TabEntry) -> usize {
        let index = self.tab.len();
        let block_index = 0;  // Always use global block
        
        let mut prev_same_type: Option<usize> = None;
        let mut current = self.btab[block_index].last;
        
        while current > 0 {
            if self.tab[current].obj == entry.obj {
                prev_same_type = Some(current);
                break;
            }
            current = self.tab[current].link.unwrap_or(0);
        }
        
        entry.link = prev_same_type;  // Previous entry of same type (or None)
        entry.level = 0;  // Force global level
        
        self.tab.push(entry);
        index
    }
    
    /// Check if an identifier is a built-in procedure or constant
    /// recognized by lexer but not stored in symbol table
    pub fn is_builtin(&self, name: &str) -> bool {
        matches!(name, "writeln" | "write" | "readln" | "read" | "true" | "false")
    }
    
    /// Lookup identifier only in current scope (for redeclaration checking)
    pub fn lookup_current_scope(&self, name: &str) -> Option<usize> {
        let level = self.current_level();
        let block_index = self.display[level];
        let mut current = self.btab[block_index].last;  // Points to most recent identifier
        
        while current > 0 {
            if self.tab[current].name == name {
                return Some(current);
            }
            current = self.tab[current].link.unwrap_or(0);
        }
        
        None
    }
    
    /// Add an array type to atab
    pub fn insert_array(&mut self, entry: ATabEntry) -> usize {
        let index = self.atab.len();
        self.atab.push(entry);
        index
    }
    
    /// Get current block index
    pub fn current_block(&self) -> usize {
        self.display[self.current_level()]
    }
    
    /// Update variable size for current block
    pub fn add_var_size(&mut self, size: usize) {
        let block_index = self.current_block();
        self.btab[block_index].var_size += size;
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Symbol Table (tab):")?;
        writeln!(f, "{:<4} {:<15} {:<12} {:<10} {:<5} {:<4} {:<4} {:<5} {:<5}", 
                 "idx", "name", "obj", "type", "ref", "nrm", "lev", "adr", "link")?;
        writeln!(f, "{}", "-".repeat(75))?;
        
        for (i, entry) in self.tab.iter().enumerate() {
            writeln!(
                f,
                "{:<4} {:<15} {:<12} {:<10} {:<5} {:<4} {:<4} {:<5} {:<5}",
                i,
                entry.name,
                format!("{}", entry.obj),
                entry.data_type.to_numeric(),  // Use numeric representation
                entry.ref_index.map_or("-".to_string(), |r| r.to_string()),
                if entry.normal { "1" } else { "0" },
                entry.level,
                entry.address,
                entry.link.map_or("-".to_string(), |l| l.to_string())
            )?;
        }
        
        writeln!(f, "\nBlock Table (btab):")?;
        writeln!(f, "{:<4} {:<6} {:<6} {:<6} {:<6}", "idx", "last", "lpar", "psze", "vsze")?;
        writeln!(f, "{}", "-".repeat(30))?;
        
        for (i, entry) in self.btab.iter().enumerate() {
            writeln!(
                f,
                "{:<4} {:<6} {:<6} {:<6} {:<6}",
                i, entry.last, entry.last_par, entry.param_size, entry.var_size
            )?;
        }
        
        if !self.atab.is_empty() {
            writeln!(f, "\nArray Table (atab):")?;
            writeln!(f, "{:<4} {:<10} {:<10} {:<5} {:<6} {:<6} {:<6} {:<6}", 
                     "idx", "xtyp", "etyp", "eref", "low", "high", "elsz", "size")?;
            writeln!(f, "{}", "-".repeat(60))?;
            
            for (i, entry) in self.atab.iter().enumerate() {
                writeln!(
                    f,
                    "{:<4} {:<10} {:<10} {:<5} {:<6} {:<6} {:<6} {:<6}",
                    i,
                    format!("{}", entry.index_type),
                    format!("{}", entry.element_type),
                    entry.element_ref.map_or("-".to_string(), |r| r.to_string()),
                    entry.low_bound,
                    entry.high_bound,
                    entry.element_size,
                    entry.total_size
                )?;
            }
        }
        
        Ok(())
    }
}
