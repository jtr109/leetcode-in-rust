/*
 * https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3737/
 *
 * 这道题的核心是使用 max binary heap 而非 vector 来处理数据，避免多次轮训目标数组判断最大数。
 *
 * 根据题目的特点其实是可以找到线索的：
 *
 * 1. 关注的是最大值（实施过程中会发现也关注第二大的值，即 max numbers 问题）
 * 2. 不关注目标数组的序号，即可以通过其他更好的数据结构处理，忽略目标结构
 *
 * 另外在性能方面，如果最大值在做完减操作后还是最大的，插入和取出该值的操作没有意义，却会触发两次堆排序。所以可以判断如果处理后的最大值大于第二大的值，那么就继续做减处理。
 *
 * 除此之外需要处理的都是一些边界问题，如：
 *
 * * 只有一个数字的时候该怎么处理
 */

use std::collections::BinaryHeap;

pub struct Solution {}

impl<'a> Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            match target.iter().nth(0) {
                Some(1) => return true,
                _ => return false,
            }
        }
        let mut heap = BinaryHeap::new();
        let mut sum = 0 as i64;
        for &e in target.iter() {
            heap.push(e as i64);
            sum += e as i64;
        }
        loop {
            // 提取堆中最大的数字
            let mut max = heap.pop().unwrap();
            if max == 1 {
                return true;
            }
            let second_max = *heap.peek().unwrap();
            loop {
                // 处理最大数字直到它不是最大的
                let stuff = 2 * max - sum; // 计划插入对象
                if stuff < 1 {
                    return false;
                } else if stuff == 1 && second_max == 1 {
                    return true;
                }
                sum -= max - stuff;
                if stuff < second_max {
                    heap.push(stuff);
                    break;
                }
                max = stuff;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let target = vec![9, 3, 5];
        assert!(Solution::is_possible(target));
    }

    #[test]
    fn example_2() {
        let target = vec![1, 1, 1, 2];
        assert!(!Solution::is_possible(target));
    }

    #[test]
    fn example_3() {
        let target = vec![8, 5];
        assert!(Solution::is_possible(target));
    }

    #[test]
    fn submission_1() {
        let target = vec![5, 2];
        assert!(Solution::is_possible(target));
    }

    #[test]
    fn submission_2() {
        let target = vec![9, 9, 9];
        assert!(!Solution::is_possible(target));
    }

    #[test]
    fn submission_3() {
        let target = vec![1, 1, 61, 9, 17];
        assert!(Solution::is_possible(target));
    }

    #[test]
    fn submission_4() {
        let target = vec![2];
        assert!(!Solution::is_possible(target));
    }

    #[test]
    fn submission_5() {
        let target = vec![
            835647834, 503037935, 773002076, 731298404, 903645595, 488189634, 319785391, 111546683,
            609144970, 415205491, 685900245, 878508756, 236413773, 991053691, 947595808, 913014938,
            533038855, 88376603, 436545178, 983496954, 122530918, 346542007, 55893465, 472984628,
            347337093, 322587100, 552866643, 609759356, 288893937, 471774337, 465491640, 783022406,
            699817530, 340584553, 663909719, 651419106, 846593366, 952609573, 912379694, 661318302,
            538633771, 745093202, 753577352, 60741272, 800245613, 228718955, 314289253, 384902244,
            834091366, 330486268, 832528567, 405339553, 667374764, 477631332, 458512135, 281436435,
            301856749, 331522322, 439316110, 65782135, 313620054, 377064760, 689776101, 453352404,
            739524725, 113039032, 403624252, 864855957, 816177580, 472331359, 702356269, 634580725,
            79566086, 723272803, 484129094, 785382934, 519691527, 303358848, 673141033, 900376058,
            640090231, 332948759, 3578533, 603932450, 300252459, 455172786, 398327644, 667611961,
            579527425, 847780358, 251487657, 239105566, 949519075, 816672375, 569680210, 522034786,
            433488047, 339341869, 323607606, 695508020, 671840506, 403376732, 437224135, 704526427,
            733331510, 566004060, 190603334, 401309800, 135615700, 480920888, 880495868, 394755529,
            754131300, 980354442, 940475050, 455976643, 26150213, 620704469, 305714399, 452126616,
            331922227, 285017717, 547688077, 571029451, 580099665, 888037179, 116069830, 492086251,
            460673303, 652403652, 529457074, 959765712, 884239314, 707976976, 421820667, 102910899,
            850649035, 332557694, 156833539, 193014693, 712889809, 65402492, 81873875, 205826928,
            906576787, 536580780, 98764481, 614216242, 724741649, 926963940, 415525579, 707467586,
            670685849, 645316339, 663012408, 451540628, 190074574, 100279961, 373676876, 108645392,
            439186589, 289025528, 317751557, 696247292, 252536329, 524764647, 664019135, 681731166,
            697044386, 721419536, 89117662, 386734310, 933623270, 18288756, 858592177, 843786785,
            267433641, 104266328, 816027769, 103604036, 874219314, 626912072, 643973051, 138646542,
            930635468, 638385362, 666376936, 75446650, 725532251, 558802301, 898021675, 602462415,
            287100257, 103256759, 241469306, 418203100, 359820338, 439745508, 711532895, 913171524,
            828285226, 490697952, 94810193, 272869408, 26560972, 979834901, 5721504, 185048349,
            42545696, 396115122, 904051513, 220906107, 738993493, 588839845, 742987869, 641939237,
            20134580, 613427664, 760712842, 725934923, 233138479, 694808775, 504699750, 45306699,
            928921991, 348537469, 341584856, 314823473, 323200569, 5906520, 590908812, 7058731,
            612527161, 95929757, 923027538, 504340570, 108267579, 981049825, 3621484, 268758794,
            155138552, 967177696, 154587439, 226488698, 435889878, 282062129, 837804113, 874696398,
            704916352, 672370979, 429701338, 699214790, 311303195, 947796876, 50395096, 943382310,
            264134145, 110060599, 662341190, 707099896, 778109433, 209892183, 129970328, 614555781,
            65561327, 723924768, 308208692, 907464540, 511861513, 664166961, 278299804, 383189150,
            305144703, 399367644, 745158781, 611505481, 788035592, 958094209, 4691422, 982804694,
            182195783, 644444281, 888532003, 913888496, 781449985, 252740240, 661510236, 998487245,
            284318090, 646237565, 822471400, 371153017, 988783, 234143485, 906623027, 265667900,
            88762375, 654203306, 124205686, 724165887, 715879556, 778678123, 201533689, 241172717,
            220243793, 300732194, 834092268, 546274723, 978426161, 544966162, 860580567, 833904274,
            408002579, 683573852, 414196298, 567536223, 401758309, 449922489, 264257881, 552000887,
            313883439, 121158141, 816111867, 426696045, 166578559, 869183250, 677763442, 449243505,
            446613544, 618933881, 765618361, 782239559, 139564825, 17361518, 375149549, 752695711,
            170774896, 24223979, 929368291, 714737788, 968883177, 418171397, 28938451, 530535587,
            831491933, 388232832, 375471966, 501547932, 42057135, 105907929, 319325198, 866969758,
            883623220, 636639049, 98250237, 119670840, 810521959, 994818843, 979182684, 698619283,
            306787611, 79779236, 316097105, 20407838, 246365554, 405157606, 746584313, 338447287,
            956653822, 157277229, 382734547, 919850823, 17698838, 400783732, 846595424, 820236688,
            844188244, 592045378, 664969413, 199985888, 986952695, 924241714, 545487784, 487539172,
            147186879, 945151212, 958877753, 669099388, 403471548, 499546716, 915071435, 897865485,
            892380510, 328284017, 649646782, 564916899, 126507447, 341928577, 916918420, 335542645,
            562151880, 341948589, 153658487, 915169176, 189949851, 956608598, 92763236, 671268832,
            829439855, 198464213, 96704707, 504412819, 126161975, 935193455, 513168023, 134286447,
            385405295, 615567547, 17991890, 790707341, 991003956, 917307082, 145826213, 725182412,
            707571456, 193431873, 694550855, 988821536, 296476226, 166333863, 595537769, 579249117,
            788071113, 445822525, 312987370, 690589050, 707167073, 734770753, 836025747, 398024391,
            614034356, 19892950, 177961692, 901857208, 570179250, 880745468, 628351955, 471663624,
            537067058, 620422588, 801133337, 443608528, 655426152, 630822673, 323931531, 616054005,
            715419230, 344542774, 569479396, 46182177, 115731968, 615621055, 927437470, 415304055,
            613134814, 593402338, 555887984, 424883671, 737555229, 580850096, 206291958, 408415618,
            568495923, 383046806, 331153888, 630013482, 639299672, 932192570, 426968629, 665396997,
            377560684, 425576288, 499413833, 304016029, 435936610, 890997684, 37462641, 308478314,
            648743689, 968679592, 6563304, 528867706, 815369648, 699526086, 854594514, 77461257,
            912978080, 437849059, 584065006, 448033159, 851418436, 275202511, 933127854, 57918210,
            936121297, 728315416, 493270959, 635640085, 751603865, 815510463, 322128401, 817518864,
            113319096, 719706704, 151675058, 579563270, 817496583, 331259914, 304828788, 832758432,
            514693374, 628452536, 859015740, 54698207, 502378937, 925823748, 468799569, 912430738,
            503849861, 155092332, 42145830, 16914543, 954818713, 944191613, 753948564, 101131208,
            628620169, 514926770, 674609079, 477754503, 226746575, 399775590, 317837974, 685000591,
            380154246, 862619940, 156768091, 907663822, 640534228, 509892852, 140342102, 997418526,
            552535460, 430179820, 454222315, 646598713, 14409817, 280310343, 324342456, 625965172,
            814280682, 974660429, 122025967, 697839974, 290110820, 878936393, 874327021, 252896959,
            368474189, 596100537, 759459589, 74112139, 496075772, 660681195, 476919942, 732314798,
            881551587, 600771213, 916355905, 62502174, 722694262, 846208181, 235701369, 377851030,
            444635218, 905821472, 6007590, 14543647, 806494416, 995077069, 714758118, 844053358,
            554781732, 772216007, 970855797, 411569324, 513182664, 137085083, 204095216, 632442670,
            843815967, 289982568, 631746100, 825138049, 349162102, 5326115, 207201935, 835205999,
            161469959, 312314799, 644367317, 122663749, 602404965, 763996480, 974548439, 732304994,
            808596406, 180421152, 975264158, 122323736, 966123769, 50110342, 744171007, 561228509,
            513803739, 631885254, 848740278, 38362310, 334317889, 407483295, 704719903, 793514069,
            3850010, 47429717, 332085264, 296925009, 179969654, 285965854, 188329006, 353044452,
            907132439, 173498729, 971005353, 886796300, 374137584, 167007939, 257311427, 69578518,
            986207933, 820577598, 762893591, 855603877, 154159499, 81230719, 988672413, 560039653,
            187206037, 124634358, 756546188, 562543979, 382863463, 791317508, 638677331, 472652962,
            704857946, 755439531, 209470628, 413904724, 236402946, 980178476, 925780627, 624361832,
            986373739, 865597279, 733945791, 40519093, 346073968, 531933636, 452742332, 388931131,
            509971886, 203046081, 86591499, 107265603, 170834917, 222658017, 931432329, 545903464,
            244840320, 750824817, 665373361, 418717528, 723513553, 572491457, 458224851, 688126547,
            872604574, 477404041, 810687226, 902428227, 809180291, 181691685, 593946617, 443990866,
            886931195, 671951824, 117916938, 88508493, 101987861, 998221579, 302729532, 637782794,
            713816070, 909221425, 478649015, 947515758, 588376423, 102208139, 387839447, 126033549,
            266213171, 286274226, 201637730, 911783168, 261943681, 622500013, 456472374, 708804967,
            489562877, 836248357, 622073857, 432626926, 157337505, 416320771, 436152162, 592575425,
            42633820, 518316330, 29165774, 254298082, 747662083, 309417930, 635421959, 306846172,
        ];
        assert!(!Solution::is_possible(target));
    }
}
