using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Energy : MonoBehaviour
{
    static public GameObject prefab;

    public float _energy;
    public float energy {
        get => _energy;
        set
        {
            _energy = value;
            var color = sprite.color;
            color.a = (-1f / (_energy / 20f + 1f)) + 1f;
            sprite.color = color;
        }
    }

    SpriteRenderer sprite;

    private void OnEnable() {
        prefab = Resources.Load<GameObject>("Energy");
    }

    void Awake()
    {

        sprite = GetComponent<SpriteRenderer>();
        energy = Settings.inst.energy.value.sample();
    }

    private void Start() {
        var body = GetComponent<Rigidbody2D>();
        body.AddForce(new Vector2(Settings.inst.energy.velocity.sample(), Settings.inst.energy.velocity.sample()));
    }

    private void FixedUpdate() {
        //Cell genesis
        if (energy >= Settings.inst.energy.toCellThresh) {
            //Random weighted sample to get new cell prefab
            GameObject newCellPrefab = null;
            while (newCellPrefab == null) {
                int i = Random.Range(0, Cell.prefabs.Count);
                if (Random.value < Cell.prefabs[i].chance) {
                    newCellPrefab = Cell.prefabs[i].prefab;
                }
            }
            //Instantiate new cell and delete self
            var newCell = GameObject.Instantiate(
                newCellPrefab, 
                transform.position,
                Quaternion.identity
            );
            newCell.transform.localScale = Settings.inst.energy.scale;
            newCell.GetComponent<Cell>().energy = energy;

            GameObject.Destroy(gameObject);
        }
    }

    void OnCollisionEnter2D(Collision2D col)
    {
        if (gameObject.activeSelf == false) {
            return;
        }

        var energyObj = col.gameObject.GetComponent<Energy>();
        if (energyObj == null) {
            return;
        }

        col.gameObject.SetActive(false);
        energy += energyObj.energy;
        GameObject.Destroy(col.gameObject);
    }
}
