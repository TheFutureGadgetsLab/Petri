using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Cell : MonoBehaviour
{
    public Dictionary<GameObject, Bond> joints = new Dictionary<GameObject, Bond>();
    protected new Rigidbody2D rigidbody;
    protected GameObject bondPrefab;

    public float energy = 0.0f;

    private CellParams cellConfig;
    private GameObject EnergyPrefab;

    public uint organismID;

    protected void Awake() {
        organismID = (uint)Random.Range(int.MinValue, int.MaxValue);
        cellConfig = GameObject.Find("Settings").GetComponent<Settings>().cellParams;
        rigidbody = GetComponent<Rigidbody2D>();
        bondPrefab = Resources.Load<GameObject>("Bond");
        EnergyPrefab = Resources.Load<GameObject>("Energy");
    }

    protected void FixedUpdate()
    {
        foreach (var joint in joints) {
            Cell neighbor = null;

            if (joint.Key.GetComponent<Cell>())
                neighbor = joint.Key.GetComponent<Cell>();

            if (energy >= cellConfig.shareRate) {
                neighbor.energy += cellConfig.shareRate;
                energy -= cellConfig.shareRate;
            }
        }

        if (energy < cellConfig.minEnergy) {
            destabilize();
        }
    }

    void destabilize()
    {
        foreach (var joint in joints) {
            var cell = joint.Key.GetComponent<Cell>();
            GameObject.Destroy(joint.Value.gameObject);
            cell.joints.Remove(gameObject);
        }
        var newEnergy = GameObject.Instantiate(
            EnergyPrefab,
            transform.position,
            Quaternion.identity
        );
        newEnergy.GetComponent<Energy>().energy = energy;

        joints.Clear();
        GameObject.Destroy(gameObject);
    }

    // Called when colliding with another rigidbody
    void OnCollisionEnter2D(Collision2D col)
    {
        handleCollision(col);
        handleEnergy(col);
    }

    void handleCollision(Collision2D col)
    {
        var otherCell = col.gameObject.GetComponent<Cell>();
        if (otherCell == null) {
            return;
        }

        //Bond formation
        if (joints.Count < cellConfig.maxBonds
            && col.relativeVelocity.magnitude > cellConfig.bondForce
            && otherCell.organismID != organismID)
        {
            var obj = GameObject.Instantiate(bondPrefab, Vector3.zero, Quaternion.identity);
            obj.transform.parent = transform;
            var cellJoint = obj.GetComponent<Bond>();
            cellJoint.transform.localPosition = Vector3.zero;
            cellJoint.ConnectTo(otherCell);
            joints.Add(col.gameObject, cellJoint);
            otherCell.joints.Add(gameObject, cellJoint);

            otherCell.organismID = organismID;
        }
    }

    void handleEnergy(Collision2D col)
    {
        var energyObj = col.gameObject.GetComponent<Energy>();
        if (energyObj == null) {
            return;
        }

        energy += energyObj.energy;
        
        GameObject.Destroy(col.gameObject);
    }
}
