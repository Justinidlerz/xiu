xiu::锈! {
    外 箱 锈;

    用 中::仓::典 作 典;

    性 键值 {
        函 写(&身, 键: 串, 值: 串);
        函 读(&身, 键: 串) -> 果<或<&串>, 串>;
    }

    静 变 籍: 或<典<串, 串>> = 无;

    构 实;

    阐 键值 为 实 {
        函 写(&身, 键: 串, 值: 串) {
            定 书 = 危 {
                籍.取入(标::准)
            };
            书.入(键, 值);
        }

        函 读(&身, 键: 串) -> 果<或<&串>, 串> {
            若 定 有(书) = 危 { 籍.作引() } {
                好(书.取(&键))
            } 否则 {
                错("未之有也".进())
            }
        }
    }

    公(箱) 函 可能(i: u32) -> 或<果<u32, 串>> {
        若 i % 2 == 1 {
            若 i == 42 {
                有(错(串::从("非也")))
            } 否则 {
                有(好(33))
            }
        } 否则 {
            无
        }
    }

    另 函 例甲() {

    }

    另 函 例乙() {
        例甲().等;
    }

    函 主() {
        定 变 物 = 31;

        配 物 {
            42 => {
                印!("然")
            }
            _ => 印!("非也")
        }

        令 甲 在 0..10 {
            定 量 = 环 {
                断 甲;
            };

            当 无或 物 < 量 {
                物 += 1;
            }

            物 = 若 定 有(物) = 可能(甲) {
                物.解()
            } 否则 {
                12
            };
        }
    }
}