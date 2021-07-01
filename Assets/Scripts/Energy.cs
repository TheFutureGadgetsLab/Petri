using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Energy : MonoBehaviour
{
    public float _energy;
    public static List<GameObject> instances = new List<GameObject>();

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

    List<GameObject> CellPrefabs;

    EnergyParams energyConfig;
    SpriteRenderer sprite;

    void Awake()
    {
        CellPrefabs = new List<GameObject>(){
            Resources.Load<GameObject>("Cell"),
            Resources.Load<GameObject>("Propulsion"),
            Resources.Load<GameObject>("Weapon"),
        };

        sprite = GetComponent<SpriteRenderer>();
        energyConfig = GameObject.Find("Settings").GetComponent<Settings>().energyParams;
        energy = energyConfig.value.sample();
    }

    private void Start() {
        var body = GetComponent<Rigidbody2D>();
        body.AddForce(new Vector2(energyConfig.velocity.sample(), energyConfig.velocity.sample()));
    }

    private void FixedUpdate() {
        if (energy >= energyConfig.toCellThresh) {
            var newCell = GameObject.Instantiate(
                CellPrefabs[(int)Random.Range(0, CellPrefabs.Count)], 
                transform.position,
                Quaternion.identity
            );
            newCell.transform.localScale = energyConfig.scale;
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

    // Add to static list of instances on enable
    private void OnEnable() {
        instances.Add(transform.gameObject);
    }

    // Remove from static list of instances on disable
    private void OnDisable() {
        instances.Remove(transform.gameObject);
    }
}
