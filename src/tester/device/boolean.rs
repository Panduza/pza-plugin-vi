use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Instance,
};

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("boolean").finish().await;
    log_debug_mount_start!(class.logger());

    //
    //
    let att_boolean_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"read command

## Qu'est-ce que le Lorem Ipsum?

Le Lorem Ipsum est simplement du faux texte employé dans la composition et
la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de l'imprimerie depuis les années 1500, quand un imprimeur anonyme assembla ensemble des morceaux de texte pour réaliser un livre spécimen de polices de texte. Il n'a pas fait que survivre cinq siècles, mais s'est aussi adapté à la bureautique informatique, sans que son contenu n'en soit modifié. 
Il a été popularisé dans les années 1960 grâce à la vente de feuilles Letraset contenant des passages du Lorem Ipsum, et, plus récemment, par son inclusion dans des applications 
de mise en page de texte, comme Aldus PageMaker.
        
## D'où vient-il?

Contrairement à une opinion répandue, le Lorem Ipsum n'est pas simplement du texte 
aléatoire. Il trouve ses racines dans une oeuvre de la littérature latine classique datant de 45 av. J.-C., le rendant vieux de 2000 ans. Un professeur du Hampden-Sydney College, en Virginie, s'est intéressé à un des mots latins les plus obscurs, consectetur, extrait d'un passage du Lorem Ipsum, 
et en étudiant tous les usages de ce mot dans la littérature classique.
        "#)
        .start_as_boolean()
        .await?;
    att_boolean_ro.set(false).await?;

    //
    //
    let mut att_boolean_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(
            r#"write command
    # Heading level 1

    I just love **bold text**.
    I just love __bold text__.
    Love**is**bold

    ## Heading level 2

    Italicized text is the *cat's meow*.
    Italicized text is the _cat's meow_.

    ### Heading level 3

    > Dorothy followed her through many of the beautiful rooms in her castle.
    > Dorothy followed her through many of the beautiful rooms in her castle.
    > - Revenue was off the chart.
    > - Profits were higher than ever.

            "#,
        )
        .start_as_boolean()
        .await?;

    //
    //
    tokio::spawn(async move {
        loop {
            att_boolean_wo.wait_for_commands().await;
            while let Some(command) = att_boolean_wo.pop().await {
                log_info!(att_boolean_wo.logger(), "command recieved - {:?}", command);
                att_boolean_ro.set(command).await.unwrap();
            }
        }
    });

    //
    //
    let mut att_boolean_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(
            r#"read write command

    #### Heading level 4

    1. First item
    2. Second item
    3. Third item
    4. Fourth item

    I really like using Markdown.
    I think I'll use it to format all of my documents from now on.

    My favorite search engine is [Duck Duck Go](https://duckduckgo.com).

            "#,
        )
        .start_as_boolean()
        .await?;
    att_boolean_rw.set(false).await?;

    //
    //
    tokio::spawn(async move {
        loop {
            att_boolean_rw.wait_for_commands().await;
            while let Some(command) = att_boolean_rw.pop().await {
                log_info!(att_boolean_rw.logger(), "command recieved - {:?}", command);
                att_boolean_rw.set(command).await.unwrap();
            }
        }
    });

    log_debug_mount_end!(class.logger());
    Ok(())
}
