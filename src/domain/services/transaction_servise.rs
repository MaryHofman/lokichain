use crate::domain::models::transaction::Transaction;
use crate::domain::models::transaction::TransactionStatus;

use sha2::{Sha256, Digest};

impl Transaction {
   //создание транзакции
    pub fn new(sender: String, receiver: String, amount: f64, timestamp: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            signature: None,
            transaction_id: None,
            status: TransactionStatus::Pending,
        }
    }
    //подпись транзакции
    pub fn sign_transaction(&mut self, private_key: &str) {
        let data = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.timestamp);
        self.signature = Some(self.sign_data(&data, private_key));
    }
    //верификация транзакции
    pub fn verify_signature(&self, public_key: &str) -> bool {
        if let Some(signature) = &self.signature {
            let data = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.timestamp);
            return self.verify_signature_logic(&data, signature, public_key);
        }
        false
    }

    //хэш транзакции
    pub fn generate_transaction_hash(&mut self) {
        let data = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.timestamp);
        self.transaction_id = Some(self.hash_data(&data));
    }

   

    fn sign_data(&self, data: &str, private_key: &str) -> String {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_str(private_key).expect("Неверный закрытый ключ");
        let message = Message::from_slice(data.as_bytes()).expect("Неверное сообщение");
        let sig = secp.sign(&message, &secret_key);
        format!("{:x}", sig)
    }

    fn verify_signature_logic(&self, data: &str, signature: &str, public_key: &str) -> bool {
        let secp = Secp256k1::new();
        let pub_key = PublicKey::from_str(public_key).expect("Неверный открытый ключ");
        let message = Message::from_slice(data.as_bytes()).expect("Неверное сообщение");

        let sig = secp256k1::Signature::from_str(signature).expect("Неверная подпись");
        secp.verify(&message, &sig, &pub_key).is_ok()
    }

    fn hash_data(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{}", result)
    }

    pub fn change_status(&mut self, new_status: TransactionStatus) {
        self.status = new_status;
    }

    pub fn display_info(&self) -> String {
        let signature_status = match &self.signature {
            Some(_) => "Подписана",
            None => "Не подписана",
        };

        let transaction_id_status = match &self.transaction_id {
            Some(id) => id,
            None => "Не сгенерирован",
        };

        format!(
            "Информация о транзакции:\n\
            Отправитель: {}\n\
            Получатель: {}\n\
            Сумма: {}\n\
            Время: {}\n\
            Статус: {:?}\n\
            Подпись: {}\n\
            ID транзакции: {}\n",
            self.sender,
            self.receiver,
            self.amount,
            self.timestamp,
            self.status,
            signature_status,
            transaction_id_status
        )
    }
}

