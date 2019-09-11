import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { SecretsListComponent } from './secrets-list.component';

describe('SecretsListComponent', () => {
  let component: SecretsListComponent;
  let fixture: ComponentFixture<SecretsListComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ SecretsListComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(SecretsListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
