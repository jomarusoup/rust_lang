use std::io;

fn main() {
    // �������� ����Ͽ� ���ڿ� ������ ���� ������ ����
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Using shadowing, spaces: {}", spaces); // ���: Using shadowing, spaces: 3

    // mut�� ����Ͽ� ������ ������ ���Ҵ��ϸ� ������ Ÿ�� ���� �߻�
    let mut spaces = "   ";
    // spaces = spaces.len(); // �ּ� ó���Ͽ� ������ Ȯ���� �� �ֵ��� ��
    println!("Using mut, spaces: {}", spaces);
}

