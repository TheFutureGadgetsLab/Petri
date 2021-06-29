using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Food : MonoBehaviour
{
    public float _food;
    public static List<GameObject> instances = new List<GameObject>();

    public float food {
        get => _food;
        set
        {
            _food = value;
            var color = sprite.color;
            color.a = (-1f / (_food / 20f + 1f)) + 1f;
            sprite.color = color;
        }
    }

    List<GameObject> CellPrefabs;

    FoodParams foodConfig;
    SpriteRenderer sprite;

    void Awake()
    {
        CellPrefabs = new List<GameObject>(){
            Resources.Load<GameObject>("Cell"),
            Resources.Load<GameObject>("Propulsion"),
            Resources.Load<GameObject>("Weapon"),
        };

        sprite = GetComponent<SpriteRenderer>();
        foodConfig = GameObject.Find("Settings").GetComponent<Settings>().foodParams;
        food = foodConfig.value.sample();
    }

    private void Start() {
        var body = GetComponent<Rigidbody2D>();
        body.AddForce(new Vector2(foodConfig.velocity.sample(), foodConfig.velocity.sample()));
    }

    private void FixedUpdate() {
        if (food >= foodConfig.toCellThresh) {
            var newCell = GameObject.Instantiate(
                CellPrefabs[(int)Random.Range(0, CellPrefabs.Count)], 
                transform.position,
                Quaternion.identity
            );
            newCell.transform.localScale = foodConfig.scale;
            newCell.GetComponent<Cell>().food = food;

            GameObject.Destroy(gameObject);
        }
    }

    void OnCollisionEnter2D(Collision2D col)
    {
        if (gameObject.activeSelf == false) {
            return;
        }

        var foodObj = col.gameObject.GetComponent<Food>();
        if (foodObj == null) {
            return;
        }

        col.gameObject.SetActive(false);
        food += foodObj.food;
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
