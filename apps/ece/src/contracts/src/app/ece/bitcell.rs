use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("YourProgramIDHere");

#[program]
pub mod bitcell {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bitcell = &mut ctx.accounts.bitcell;
        bitcell.owner = *ctx.accounts.owner.key;
        bitcell.data = HashMap::new();
        bitcell.performance_metrics = HashMap::new();
        bitcell.health_status = HashMap::new();
        bitcell.historical_heartbeat = HashMap::new();
        Ok(())
    }

    pub fn update_heartbeat(ctx: Context<UpdateHeartbeat>, bitcell_id: Pubkey, heartbeat: bool, timestamp: u64) -> Result<()> {
        let bitcell = &mut ctx.accounts.bitcell;
        let historical_data = bitcell.historical_heartbeat.entry(bitcell_id).or_insert(HistoricalHeartbeat {
            timestamps: Vec::new(),
            heartbeats: Vec::new(),
        });

        historical_data.timestamps.push(timestamp);
        historical_data.heartbeats.push(heartbeat);

        Ok(())
    }

    pub fn update_organelle_health(ctx: Context<UpdateOrganelleHealth>, bitcell_id: Pubkey, organelle: String, health_status: bool, timestamp: u64) -> Result<()> {
        let bitcell = &mut ctx.accounts.bitcell;
        let health_data = bitcell.health_status.entry(bitcell_id).or_insert(HealthStatus {
            organelles: HashMap::new(),
            alerts: Vec::new(),
        });

        let organelle_health = OrganelleHealth {
            status: health_status,
            last_check: timestamp,
        };

        health_data.organelles.insert(organelle.clone(), organelle_health);
        check_for_alerts(bitcell, &bitcell_id, &organelle, &health_data);

        Ok(())
    }

    fn check_for_alerts(bitcell: &mut BitCellular, bitcell_id: &Pubkey, organelle: &String, health_data: &HealthStatus) {
        if let Some(organelle_health) = health_data.organelles.get(organelle) {
            if !organelle_health.status {
                let alert_message = format!("Alert: Organelle {} in bitcell {} is down.", organelle, bitcell_id);
                send_alert(bitcell, &alert_message);
            }
        }
    }

    fn send_alert(bitcell: &mut BitCellular, message: &str) {
        msg!(message); // Log the alert message
    }

    pub fn check_bollinger_bands(ctx: Context<CheckBollingerBands>, bitcell_id: Pubkey, benchmark: f64) -> Result<bool> {
        let (mean, std_dev) = calculate_mean_and_std_dev(ctx.accounts.bitcell, bitcell_id);
        let upper_band = mean + 2.0 * std_dev;
        let lower_band = mean - 2.0 * std_dev;

        Ok(benchmark < lower_band || benchmark > upper_band)
    }

    fn calculate_mean_and_std_dev(bitcell: &BitCellular, bitcell_id: Pubkey) -> (f64, f64) {
        if let Some(hist_data) = bitcell.historical_heartbeat.get(&bitcell_id) {
            let heartbeat_values: Vec<u64> = hist_data.heartbeats.iter().map(|&hb| hb as u64).collect();
            let mean: f64 = heartbeat_values.iter().sum::<u64>() as f64 / heartbeat_values.len() as f64;

            let variance: f64 = heartbeat_values.iter().map(|&value| {
                let diff = mean - value as f64;
                diff * diff
            }).sum::<f64>() / heartbeat_values.len() as f64;

            let std_dev = variance.sqrt();
            (mean, std_dev)
        } else {
            (0.0, 0.0)
        }
    }
}

#[account]
pub struct BitCellular {
    pub owner: Pubkey,
    pub data: HashMap<String, String>,
    pub performance_metrics: HashMap<Pubkey, PerformanceMetrics>,
    pub health_status: HashMap<Pubkey, HealthStatus>,
    pub historical_heartbeat: HashMap<Pubkey, HistoricalHeartbeat>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PerformanceMetrics {
    // fields representing performance metrics
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OrganelleHealth {
    pub status: bool,
    pub last_check: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HealthStatus {
    pub organelles: HashMap<String, OrganelleHealth>,
    pub alerts: Vec<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HistoricalHeartbeat {
    pub timestamps: Vec<u64>,
    pub heartbeats: Vec<bool>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + std::mem::size_of::<BitCellular>())]
    pub bitcell: Account<'info, BitCellular>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateHeartbeat<'info> {
    #[account(mut)]
    pub bitcell: Account<'info, BitCellular>,
}

#[derive(Accounts)]
pub struct UpdateOrganelleHealth<'info> {
    #[account(mut)]
    pub bitcell: Account<'info, BitCellular>,
}

#[derive(Accounts)]
pub struct CheckBollingerBands<'info> {
    #[account(mut)]
    pub bitcell: Account<'info, BitCellular>,
}