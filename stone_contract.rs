token::MintTo {
                        mint: ctx.accounts.mint.to_account_info(),
                        to: ctx.accounts.observer_token_account.to_account_info(),
                        authority: ctx.accounts.authority.to_account_info(),
                    },
                    &[&[b"admin_authority", &[254]]],
                ),
                amount,
            )?;
        }
        
        emit!(ObserverRewardEvent {
            observer: ctx.accounts.observer.key(),
            amount,
            is_nft,
        });
        
        Ok(())
    }

    // === ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ ===

    // Генерация ID для нового генома
    fn get_next_genome_id() -> u64 {
        // В реальной имплементации это может быть счетчик в глобальном стейте
        let clock = Clock::get().unwrap();
        clock.slot % 1_000_000
    }

    // Генерация ID для нового камня
    fn get_next_stone_id() -> u64 {
        // В реальной имплементации это может быть счетчик в глобальном стейте
        let clock = Clock::get().unwrap();
        clock.slot
    }

    // Проверка, что локация камня находится в пределах радиуса генома
    fn is_within_genome(stone_geo: &GeoData, genome_center: &GeoData, genome_radius: u32) -> bool {
        let distance = calculate_distance(
            stone_geo.latitude, stone_geo.longitude,
            genome_center.latitude, genome_center.longitude,
        );
        
        distance <= genome_radius as f64
    }

    // Расчет расстояния между двумя координатами (формула гаверсинуса)
    fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        const EARTH_RADIUS: f64 = 6371000.0; // Радиус Земли в метрах
        
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();
        let delta_lat = (lat2 - lat1).to_radians();
        let delta_lon = (lon2 - lon1).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) + 
                lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        EARTH_RADIUS * c
    }

    // Верификация доказательства геолокации
    fn verify_geo_proof(location: &GeoData, proof: &GeoProof, oracle: Pubkey) -> Result<()> {
        // В реальном контракте здесь была бы проверка подписи оракула
        // и проверка временной метки (не слишком старая)
        
        // Пример проверки временной метки (не старше 1 часа)
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time - proof.timestamp < 3600,
            ErrorCode::GeoProofExpired
        );
        
        // Проверка, что оракул авторизован
        require!(
            is_authorized_oracle(oracle),
            ErrorCode::UnauthorizedOracle
        );
        
        Ok(())
    }

    // Проверка, авторизован ли оракул
    fn is_authorized_oracle(oracle: Pubkey) -> bool {
        // В реальной имплементации здесь был бы список авторизованных оракулов
        // Для примера возвращаем true
        true
    }

    // Поиск донора споры по геному
    fn find_spore_donor(genome: [u8; 32]) -> Option<Pubkey> {
        // В реальной имплементации здесь был бы поиск по базе данных
        // Для примера возвращаем None
        None
    }

    // Получение владельца генома по ID
    fn get_genome_owner(genome_id: u64) -> Result<Pubkey> {
        // В реальной имплементации здесь был бы поиск в стейте
        // Для примера возвращаем ошибку
        err!(ErrorCode::GenomeNotFound)
    }

    // Проверка, является ли адрес администратором
    fn is_admin(address: Pubkey) -> bool {
        // В реальной имплементации здесь был бы список администраторов
        // Для примера возвращаем false
        false
    }

    // Расчет бонуса роста мха от экологических данных
    fn calculate_env_bonus(env: &EnvData) -> u8 {
        // Пример алгоритма расчета бонуса на основе экологических данных
        let temp_bonus = if env.temperature > 15 && env.temperature < 30 { 1 } else { 0 };
        let humid_bonus = if env.humidity > 60 && env.humidity < 90 { 1 } else { 0 };
        let air_bonus = if env.air_quality > 70 { 1 } else { 0 };
        let light_bonus = if env.light_level > 50 { 1 } else { 0 };
        
        temp_bonus + humid_bonus + air_bonus + light_bonus
    }

    // Расчет количества токенов MOSS для награды
    fn calculate_moss_rewards(stone: &Stone) -> u64 {
        // Базовая награда
        let base_reward = 10;
        
        // Бонус от уровня мха
        let moss_bonus = stone.moss_level as u64 / 10;
        
        // Бонус за Genesis камень
        let type_bonus = match stone.stone_type {
            StoneType::Genesis => 5,
            StoneType::Astral => 0,
        };
        
        // Бонус от количества отзывов
        let review_bonus = stone.reviews.len() as u64;
        
        // Общая награда
        base_reward + moss_bonus + type_bonus + review_bonus
    }

    // Создание метаданных для NFT через Metaplex
    fn create_metadata(
        metadata_account: AccountInfo,
        mint: AccountInfo,
        mint_authority: AccountInfo,
        payer: AccountInfo,
        token_metadata_program: AccountInfo,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // Вызов CPI к программе метаданных Metaplex
        // Здесь должна быть реализация CPI к Metaplex
        // Для простоты примера не реализовано полностью
        
        Ok(())
    }
}

// === СОБЫТИЯ ===

#[event]
pub struct GenomeCreatedEvent {
    pub genome_id: u64,
    pub owner: Pubkey,
    pub center_latitude: f64,
    pub center_longitude: f64,
    pub radius: u32,
}

#[event]
pub struct StoneCreatedEvent {
    pub stone_id: u64,
    pub stone_type: u8, // 0 = Genesis, 1 = Astral
    pub creator: Pubkey,
    pub owner: Pubkey,
    pub genome_id: u64,
    pub latitude: f64,
    pub longitude: f64,
}

#[event]
pub struct AstralMintedEvent {
    pub genesis_id: u64,
    pub astral_id: u64,
    pub buyer: Pubkey,
    pub payment_amount: u64,
}

#[event]
pub struct StoneActivatedEvent {
    pub stone_id: u64,
    pub activator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MossHarvestedEvent {
    pub stone_id: u64,
    pub owner: Pubkey,
    pub moss_level: u8,
    pub tokens_minted: u64,
}

#[event]
pub struct SporeSentEvent {
    pub source_stone_id: u64,
    pub target_stone_id: u64,
    pub sender: Pubkey,
}

#[event]
pub struct ObserverRewardEvent {
    pub observer: Pubkey,
    pub amount: u64,
    pub is_nft: bool,
}

// === КОДЫ ОШИБОК ===

#[error_code]
pub enum ErrorCode {
    #[msg("Неверные параметры")]
    InvalidParameters,
    
    #[msg("Локация за пределами территории генома")]
    LocationOutOfBounds,
    
    #[msg("Слишком высокий налог территории")]
    TaxesTooHigh,
    
    #[msg("Камень не является Genesis")]
    NotGenesis,
    
    #[msg("Камень уже активирован")]
    AlreadyActivated,
    
    #[msg("Геном не найден")]
    GenomeNotFound,
    
    #[msg("Неверный статус камня")]
    InvalidState,
    
    #[msg("Геодоказательство устарело")]
    GeoProofExpired,
    
    #[msg("Неавторизованный оракул")]
    UnauthorizedOracle,
    
    #[msg("Слишком рано для сбора мха")]
    HarvestTooSoon,
    
    #[msg("Мох слишком молодой для отправки споры")]
    MossTooYoung,
    
    #[msg("Достигнут максимум спор для камня")]
    MaxSporesReached,
    
    #[msg("Спора с таким геномом уже существует")]
    DuplicateSpore,
    
    #[msg("Неавторизованный доступ")]
    Unauthorized,
}
