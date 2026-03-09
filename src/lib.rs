use anchor_lang::prelude::*;

declare_id!("J4BuPHNrFTXjniGvu5Kfr8TPxUF96kwDb2Vnx5cz7T59");

#[program]
pub mod pokedex {
    use super::*;

    pub fn iniciar_pokedex(context: Context<NuevaPokedex>, nombre_entrenador: String) -> Result <()> {
        let owner = context.accounts.owner.key();
        let pokemones: Vec<Pokemon> = Vec::new();

        context.accounts.pokedex.set_inner(Pokedex {
            owner, 
            nombre_entrenador,
            pokemones,
        });

        Ok(())
    }

    pub fn registrar_pokemon(context: Context<NuevoPokemon>, nombre:String, numero_pokedex:u16, tipo:String, nivel:u8) -> Result<()> {
        let pokemon= Pokemon{
            nombre,
            numero_pokedex,
            tipo,
            nivel,
            atrapado: true
        };

        context.accounts.pokedex.pokemones.push(pokemon);

        Ok(())
    }
    
    pub fn ver_pokemones(context: Context<NuevoPokemon>) -> Result<()> {
        msg!("los pokemones registrados son: {:#?}", context.accounts.pokedex.pokemones);

        Ok(())
    }

    pub fn liberar_pokemon(context: Context<NuevoPokemon>, nombre:String) -> Result<()> {
        let pokemones = &mut context.accounts.pokedex.pokemones;

        for pokemon in 0..pokemones.len()  {
            if pokemones[pokemon].nombre == nombre {
                pokemones.remove(pokemon);
                msg!("pokemon {nombre} liberado",);
                return Ok(());
            }  
        }
        
        Err(Errores::PokemonNoExiste.into())
    }
    
    pub fn modificar_captura(context: Context<NuevoPokemon>, nombre: String) -> Result<()> {
        let pokemones = &mut context.accounts.pokedex.pokemones;

        for pokemon in 0..pokemones.len()  {
             let estado = pokemones[pokemon].atrapado;
             
             if pokemones[pokemon].nombre == nombre {
               let nuevo_estado = !estado;
               pokemones[pokemon].atrapado = nuevo_estado;

               msg!("el pokemon {} esta atrapado: {}", nombre, nuevo_estado);
               return Ok(());
               }  
            } 

             Err(Errores::PokemonNoExiste.into())
    }
     
    
}

#[error_code]
pub enum Errores {
    #[msg("error, no eres el propietario de la cuenta")]
    NoEresElOwner,

    #[msg("error, el pokemon no esta registrado")]
    PokemonNoExiste,
}

#[account]
#[derive(InitSpace)] 
pub struct Pokedex { 
    owner: Pubkey, 

    #[max_len(60)]
    nombre_entrenador: String,

    #[max_len(20)]
    pokemones: Vec<Pokemon>,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Pokemon {
    #[max_len(60)]
    nombre: String,

    numero_pokedex: u16,
    
    #[max_len(20)]
    tipo: String,

    nivel: u8,

    atrapado: bool,
}

#[derive(Accounts)]
pub struct NuevaPokedex<'info> {
    #[account(mut)] 
    pub owner: Signer<'info>,

    #[account(
        init,
    
        payer = owner, 
        space = Pokedex::INIT_SPACE + 8, 
        seeds = [b"pokedex", owner.key().as_ref()], 
        bump 
    )]
    pub pokedex: Account<'info, Pokedex>,  

    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)] 
pub struct NuevoPokemon<'info> {
    pub owner: Signer<'info>,

    #[account(mut)] 
    pub pokedex: Account<'info, Pokedex>,
}